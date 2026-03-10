use scraper::{Html, Selector};
use uuid::Uuid;

use crate::{
    AppContext,
    entries,
    error::{AppError, AppResult},
};

pub async fn extract_entry_content(context: &AppContext, entry_id: Uuid) -> AppResult<()> {
    let target = entries::extraction_target(context, entry_id).await?;
    let html = select_html_source(context, &target).await?;
    let text = html_to_text(&html);
    if text.trim().is_empty() {
        return Err(AppError::external("content extraction produced empty text"));
    }
    entries::store_extracted_content(context, entry_id, &html, &text).await
}

async fn select_html_source(
    context: &AppContext,
    target: &entries::EntryExtractionTarget,
) -> AppResult<String> {
    if let Some(seed) = target.content_seed_html.clone().filter(|value| !value.trim().is_empty()) {
        return Ok(seed);
    }
    let url = target
        .url
        .as_ref()
        .ok_or_else(|| AppError::external("entry has no source url to extract content"))?;
    let response = context.http.get(url).send().await?;
    if !response.status().is_success() {
        return Err(AppError::external(format!("content fetch failed with {}", response.status())));
    }
    let body = response.text().await?;
    Ok(extract_primary_html(&body))
}

fn extract_primary_html(raw_html: &str) -> String {
    let document = Html::parse_document(raw_html);
    for selector in ["article", "main", "body"] {
        if let Some(node) = select_first(&document, selector) {
            return node.inner_html();
        }
    }
    raw_html.to_owned()
}

fn html_to_text(html: &str) -> String {
    let fragment = Html::parse_fragment(html);
    fragment.root_element().text().collect::<Vec<_>>().join(" ")
}

fn select_first<'a>(document: &'a Html, selector: &str) -> Option<scraper::ElementRef<'a>> {
    let selector = Selector::parse(selector).ok()?;
    document.select(&selector).next()
}

