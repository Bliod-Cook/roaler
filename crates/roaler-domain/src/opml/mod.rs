use roxmltree::Document;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    feeds,
    models::source::{CreateSourceInput, FeedSource, SourceKind},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OpmlImportInput {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OpmlExportResponse {
    pub content: String,
}

pub async fn import_opml(context: &AppContext, input: &OpmlImportInput) -> AppResult<Vec<FeedSource>> {
    let document = Document::parse(&input.content)
        .map_err(|error| AppError::validation(format!("invalid OPML: {}", error)))?;
    let mut created = Vec::new();
    for node in document.descendants().filter(|node| node.has_tag_name("outline")) {
        let Some(feed_url) = node.attribute("xmlUrl") else {
            continue;
        };
        let title = node.attribute("title").or_else(|| node.attribute("text"));
        let source = feeds::create_source(
            context,
            &CreateSourceInput {
                title: title.map(str::to_owned),
                kind: SourceKind::Rss,
                feed_url: Some(feed_url.to_owned()),
                site_url: node.attribute("htmlUrl").map(str::to_owned),
                rsshub_base_url: None,
                rsshub_route: None,
                collection_ids: vec![],
                refresh_interval_minutes: Some(30),
            },
        )
        .await?;
        created.push(source);
    }
    Ok(created)
}

pub async fn export_opml(context: &AppContext) -> AppResult<OpmlExportResponse> {
    let sources = feeds::list_sources(context).await?;
    let mut lines = vec![
        r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_owned(),
        r#"<opml version="2.0"><head><title>Roaler Export</title></head><body>"#.to_owned(),
    ];
    for source in sources {
        lines.push(format!(
            r#"<outline text="{title}" title="{title}" type="rss" xmlUrl="{feed}" htmlUrl="{site}" />"#,
            title = xml_escape(&source.title),
            feed = xml_escape(&source.feed_url),
            site = xml_escape(source.site_url.as_deref().unwrap_or(""))
        ));
    }
    lines.push("</body></opml>".to_owned());
    Ok(OpmlExportResponse {
        content: lines.join(""),
    })
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

