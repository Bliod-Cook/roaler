use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::parser::{NormalizedEntry, ParsedFeed};
use crate::{AppError, AppResult, models::SourceKind};

#[derive(Debug, Deserialize)]
pub struct JsonFeedDocument {
    title: Option<String>,
    home_page_url: Option<String>,
    feed_url: Option<String>,
    description: Option<String>,
    items: Vec<JsonFeedItem>,
}

#[derive(Debug, Deserialize)]
struct JsonFeedItem {
    id: Option<String>,
    url: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    content_text: Option<String>,
    content_html: Option<String>,
    date_published: Option<String>,
    authors: Option<Vec<JsonFeedAuthor>>,
    image: Option<String>,
    banner_image: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JsonFeedAuthor {
    name: Option<String>,
}

pub fn parse_json_feed(feed_url: &str, bytes: &[u8]) -> AppResult<ParsedFeed> {
    let document: JsonFeedDocument = serde_json::from_slice(bytes)?;
    let entries = document
        .items
        .into_iter()
        .map(parse_item)
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ParsedFeed {
        kind: SourceKind::JsonFeed,
        title: document.title.unwrap_or_else(|| "Untitled feed".to_owned()),
        description: document.description,
        site_url: document.home_page_url,
        feed_url: document.feed_url.unwrap_or_else(|| feed_url.to_owned()),
        hub_url: None,
        entries,
    })
}

fn parse_item(item: JsonFeedItem) -> AppResult<NormalizedEntry> {
    let url = item
        .url
        .ok_or_else(|| AppError::Validation("json feed item missing url".to_owned()))?;
    let title = item.title.unwrap_or_else(|| "Untitled entry".to_owned());
    let content_html = item.content_html;
    let content_text = pick_content_text(item.content_text, content_html.as_deref());
    let raw_payload = serde_json::to_value(&url)?;
    let published_at = parse_date(item.date_published)?;
    let media_urls = pick_media_urls(&item);
    Ok(NormalizedEntry::new(
        item.id,
        None,
        url,
        title,
        item.summary,
        content_html,
        content_text,
        item.authors.and_then(first_author_name),
        published_at,
        serde_json::json!(media_urls),
        raw_payload,
    ))
}

fn pick_content_text(text: Option<String>, html: Option<&str>) -> Option<String> {
    text.or_else(|| html.map(strip_html))
}

fn strip_html(html: &str) -> String {
    html.replace('<', " ").replace('>', " ")
}

fn parse_date(date: Option<String>) -> AppResult<DateTime<Utc>> {
    let value = date.unwrap_or_else(|| Utc::now().to_rfc3339());
    DateTime::parse_from_rfc3339(&value)
        .map(|parsed| parsed.with_timezone(&Utc))
        .map_err(|_| AppError::Validation("invalid json feed date".to_owned()))
}

fn first_author_name(authors: Vec<JsonFeedAuthor>) -> Option<String> {
    authors.into_iter().find_map(|author| author.name)
}

fn pick_media_urls(item: &JsonFeedItem) -> Vec<String> {
    [item.image.clone(), item.banner_image.clone()]
        .into_iter()
        .flatten()
        .collect()
}

