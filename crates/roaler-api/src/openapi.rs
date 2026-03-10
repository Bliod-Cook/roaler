use serde_json::{Value, json};

pub fn openapi_document() -> Value {
    json!({
      "openapi": "3.1.0",
      "info": {
        "title": "Roaler API",
        "version": "0.1.0"
      },
      "paths": {
        "/api/auth/bootstrap-status": { "get": { "summary": "Bootstrap status" } },
        "/api/auth/bootstrap": { "post": { "summary": "Create admin user" } },
        "/api/auth/login": { "post": { "summary": "Login admin user" } },
        "/api/auth/me": { "get": { "summary": "Current session" } },
        "/api/sources": { "get": { "summary": "List sources" }, "post": { "summary": "Create source" } },
        "/api/collections": { "get": { "summary": "List collections" }, "post": { "summary": "Create collection" } },
        "/api/entries": { "get": { "summary": "Timeline" } },
        "/api/search": { "get": { "summary": "Search entries" } },
        "/api/opml/export": { "get": { "summary": "Export OPML" } },
        "/api/opml/import": { "post": { "summary": "Import OPML" } },
        "/api/system/settings": { "get": { "summary": "Read settings" }, "put": { "summary": "Update settings" } },
        "/api/system/health": { "get": { "summary": "Health check" } },
        "/api/ai/entries/{entryId}/summary": { "post": { "summary": "Enqueue summary job" } },
        "/webhooks/websub": {
          "get": { "summary": "Confirm WebSub challenge" },
          "post": { "summary": "Receive WebSub payload" }
        }
      }
    })
}

#[cfg(test)]
mod tests {
    use super::openapi_document;

    #[test]
    fn includes_auth_path() {
        let doc = openapi_document();
        assert!(doc["paths"]["/api/auth/login"].is_object());
    }
}

