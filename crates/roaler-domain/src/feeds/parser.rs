use chrono::{DateTime, Utc};
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    error::{AppError, AppResult},
    models::source::SourceKind,
};

#[derive(Debug, Clone)]
pub struct ParsedFeed {
    pub kind: SourceKind,
    pub title: String,
    pub description: Option<String>,
    pub site_url: Option<String>,
    pub feed_url: String,
    pub hub_url: Option<String>,
    pub entries: Vec<NormalizedEntry>,
}

#[derive(Debug, Clone)]
pub struct NormalizedEntry {
    pub external_id: String,
    pub guid: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub summary: Option<String>,
    pub content_seed_html: Option<String>,
    pub author_name: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub media_json: serde_json::Value,
    pub raw_payload: serde_json::Value,
    pub dedupe_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonFeed {
    title: String,
    description: Option<String>,
    home_page_url: Option<String>,
    feed_url: Option<String>,
    items: Vec<JsonFeedItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonFeedItem {
    id: String,
    url: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    content_html: Option<String>,
    content_text: Option<String>,
    date_published: Option<String>,
    authors: Option<Vec<JsonFeedAuthor>>,
    attachments: Option<Vec<JsonFeedAttachment>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JsonFeedAuthor {
    name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JsonFeedAttachment {
    url: String,
}

pub fn parse_feed_bytes(kind: SourceKind, bytes: &[u8], feed_url: &str) -> AppResult<ParsedFeed> {
    if kind == SourceKind::JsonFeed || looks_like_json(bytes) {
        return parse_json_feed(bytes, feed_url);
    }
    parse_xml_feed(kind, bytes, feed_url)
}

fn parse_json_feed(bytes: &[u8], feed_url: &str) -> AppResult<ParsedFeed> {
    let feed: JsonFeed = serde_json::from_slice(bytes)?;
    let entries = feed
        .items
        .into_iter()
        .map(|item| {
            let id = item.id.clone();
            let title = item.title.clone().unwrap_or_else(|| "Untitled".to_owned());
            Ok(NormalizedEntry {
                external_id: id.clone(),
                guid: Some(id.clone()),
                url: item.url.clone(),
                title: title.clone(),
                summary: item.summary.clone(),
                content_seed_html: item.content_html.clone().or(item.content_text.clone()),
                author_name: item
                    .authors
                    .clone()
                    .and_then(|authors| authors.into_iter().find_map(|author| author.name)),
                published_at: item
                    .date_published
                    .clone()
                    .and_then(|value| parse_date(&value)),
                media_json: serde_json::json!(
                    item.attachments
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|attachment| attachment.url)
                        .collect::<Vec<_>>()
                ),
                raw_payload: serde_json::to_value(&item)?,
                dedupe_key: make_dedupe_key(Some(&id), item.url.as_deref(), Some(&title)),
            })
        })
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ParsedFeed {
        kind: SourceKind::JsonFeed,
        title: feed.title,
        description: feed.description,
        site_url: feed.home_page_url,
        feed_url: feed.feed_url.unwrap_or_else(|| feed_url.to_owned()),
        hub_url: None,
        entries,
    })
}

fn parse_xml_feed(kind: SourceKind, bytes: &[u8], feed_url: &str) -> AppResult<ParsedFeed> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| AppError::validation(format!("feed is not valid utf-8: {}", error)))?;
    let document = Document::parse(text)
        .map_err(|error| AppError::validation(format!("invalid xml feed: {}", error)))?;
    let root = document.root_element();
    match root.tag_name().name() {
        "feed" => parse_atom_feed(kind, root, feed_url),
        "rss" | "RDF" => parse_rss_feed(kind, root, feed_url),
        other => Err(AppError::validation(format!("unsupported feed root {}", other))),
    }
}

fn parse_atom_feed(
    kind: SourceKind,
    root: roxmltree::Node<'_, '_>,
    feed_url: &str,
) -> AppResult<ParsedFeed> {
    let title = child_text(root, "title").unwrap_or_else(|| "Untitled".to_owned());
    let description = child_text(root, "subtitle");
    let site_url = atom_link(root, "alternate");
    let hub_url = atom_link(root, "hub");
    let entries = root
        .children()
        .filter(|node| node.has_tag_name("entry"))
        .map(parse_atom_entry)
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ParsedFeed {
        kind,
        title,
        description,
        site_url,
        feed_url: feed_url.to_owned(),
        hub_url,
        entries,
    })
}

fn parse_rss_feed(
    kind: SourceKind,
    root: roxmltree::Node<'_, '_>,
    feed_url: &str,
) -> AppResult<ParsedFeed> {
    let channel = root
        .children()
        .find(|node| node.has_tag_name("channel"))
        .ok_or_else(|| AppError::validation("rss channel is missing"))?;
    let title = child_text(channel, "title").unwrap_or_else(|| "Untitled".to_owned());
    let description = child_text(channel, "description");
    let site_url = child_text(channel, "link");
    let hub_url = rss_hub_link(channel);
    let entries = channel
        .children()
        .filter(|node: &roxmltree::Node<'_, '_>| node.has_tag_name("item"))
        .map(parse_rss_entry)
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ParsedFeed {
        kind,
        title,
        description,
        site_url,
        feed_url: feed_url.to_owned(),
        hub_url,
        entries,
    })
}

fn parse_atom_entry(node: roxmltree::Node<'_, '_>) -> AppResult<NormalizedEntry> {
    let external_id = child_text(node, "id").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let url = atom_link(node, "alternate");
    let title = child_text(node, "title").unwrap_or_else(|| "Untitled".to_owned());
    let summary = child_text(node, "summary");
    let content_seed_html = child_text(node, "content").or(summary.clone());
    let author_name = node
        .children()
        .find(|child| child.has_tag_name("author"))
        .and_then(|author| child_text(author, "name"));
    let published_at = child_text(node, "published")
        .or_else(|| child_text(node, "updated"))
        .and_then(|value| parse_date(&value));
    let media = node
        .children()
        .filter(|child| child.has_tag_name("link"))
        .filter_map(|link| {
            (link.attribute("rel") == Some("enclosure"))
                .then(|| link.attribute("href").map(str::to_owned))
                .flatten()
        })
        .collect::<Vec<_>>();
    Ok(NormalizedEntry {
        external_id: external_id.clone(),
        guid: Some(external_id.clone()),
        url: url.clone(),
        title: title.clone(),
        summary,
        content_seed_html,
        author_name,
        published_at,
        media_json: serde_json::json!(media),
        raw_payload: serde_json::json!({ "id": external_id, "title": title, "url": url }),
        dedupe_key: make_dedupe_key(Some(&external_id), url.as_deref(), Some(&title)),
    })
}

fn parse_rss_entry(node: roxmltree::Node<'_, '_>) -> AppResult<NormalizedEntry> {
    let guid = child_text(node, "guid");
    let url = child_text(node, "link");
    let title = child_text(node, "title").unwrap_or_else(|| "Untitled".to_owned());
    let summary = child_text(node, "description");
    let content_seed_html = child_text(node, "encoded").or(summary.clone());
    let author_name = child_text(node, "creator").or_else(|| child_text(node, "author"));
    let published_at = child_text(node, "pubDate")
        .or_else(|| child_text(node, "date"))
        .and_then(|value| parse_date(&value));
    let external_id = guid.clone().or(url.clone()).unwrap_or_else(|| title.clone());
    let media = node
        .children()
        .filter(|child| child.has_tag_name("enclosure"))
        .filter_map(|enclosure| enclosure.attribute("url").map(str::to_owned))
        .collect::<Vec<_>>();
    Ok(NormalizedEntry {
        external_id: external_id.clone(),
        guid: guid.clone(),
        url: url.clone(),
        title: title.clone(),
        summary,
        content_seed_html,
        author_name,
        published_at,
        media_json: serde_json::json!(media),
        raw_payload: serde_json::json!({ "guid": guid, "title": title, "url": url }),
        dedupe_key: make_dedupe_key(Some(&external_id), url.as_deref(), Some(&title)),
    })
}

fn child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.tag_name().name() == name)
        .and_then(|child| child.text())
        .map(|text| text.trim().to_owned())
        .filter(|text| !text.is_empty())
}

fn atom_link(node: roxmltree::Node<'_, '_>, rel_name: &str) -> Option<String> {
    node.children()
        .filter(|child| child.has_tag_name("link"))
        .find_map(|link| {
            let rel = link.attribute("rel").unwrap_or("alternate");
            (rel == rel_name)
                .then(|| link.attribute("href").map(str::to_owned))
                .flatten()
        })
}

fn rss_hub_link(channel: roxmltree::Node<'_, '_>) -> Option<String> {
    channel
        .children()
        .filter(|child| child.tag_name().name() == "link")
        .find_map(|link| {
            let rel = link.attribute("rel")?;
            (rel == "hub")
                .then(|| link.attribute("href").map(str::to_owned))
                .flatten()
        })
}

fn looks_like_json(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .copied()
        .find(|byte| !byte.is_ascii_whitespace())
        == Some(b'{')
}

fn parse_date(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .map(|datetime| datetime.with_timezone(&Utc))
        .ok()
        .or_else(|| {
            DateTime::parse_from_rfc2822(value)
                .map(|datetime| datetime.with_timezone(&Utc))
                .ok()
        })
}

fn make_dedupe_key(guid: Option<&str>, url: Option<&str>, title: Option<&str>) -> String {
    let seed = format!(
        "{}|{}|{}",
        guid.unwrap_or_default(),
        url.unwrap_or_default(),
        title.unwrap_or_default()
    );
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::{make_dedupe_key, parse_feed_bytes};
    use crate::models::source::SourceKind;

    #[test]
    fn dedupe_key_changes_with_url() {
        let left = make_dedupe_key(Some("guid"), Some("https://a"), Some("title"));
        let right = make_dedupe_key(Some("guid"), Some("https://b"), Some("title"));
        assert_ne!(left, right);
    }

    #[test]
    fn parses_json_feed() {
        let text = br#"{"title":"Demo","items":[{"id":"1","title":"Hello"}]}"#;
        let feed = parse_feed_bytes(SourceKind::JsonFeed, text, "https://demo.test/feed")
            .expect("parse json feed");
        assert_eq!(feed.entries.len(), 1);
    }
}
