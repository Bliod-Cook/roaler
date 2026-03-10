use crate::error::{AppError, AppResult};

pub fn build_rsshub_url(base_url: &str, route: &str) -> AppResult<String> {
    let normalized_base = base_url.trim_end_matches('/');
    let normalized_route = route.trim_start_matches('/');
    if normalized_base.is_empty() || normalized_route.is_empty() {
        return Err(AppError::validation("rsshub base_url and route are required"));
    }
    Ok(format!("{}/{}", normalized_base, normalized_route))
}

#[cfg(test)]
mod tests {
    use super::build_rsshub_url;

    #[test]
    fn builds_rsshub_url() {
        let url = build_rsshub_url("https://rsshub.app/", "/github/release/openai/openai-node")
            .expect("rsshub url");
        assert_eq!(url, "https://rsshub.app/github/release/openai/openai-node");
    }
}

