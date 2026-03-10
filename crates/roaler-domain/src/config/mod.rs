use std::env;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

const DEFAULT_BIND_ADDR: &str = "0.0.0.0:8080";
const DEFAULT_SYNC_INTERVAL_SECONDS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub bind_addr: String,
    pub public_base_url: Option<String>,
    pub session_cookie_name: String,
    pub session_secret: String,
    pub files_dir: String,
    pub default_rsshub_base_url: String,
    pub sync_interval_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        Ok(Self {
            database_url: required("ROALER_DATABASE_URL")?,
            redis_url: required("ROALER_REDIS_URL")?,
            bind_addr: optional("ROALER_BIND_ADDR")
                .unwrap_or_else(|| DEFAULT_BIND_ADDR.to_owned()),
            public_base_url: optional("ROALER_PUBLIC_BASE_URL"),
            session_cookie_name: required("ROALER_SESSION_COOKIE_NAME")?,
            session_secret: required("ROALER_SESSION_SECRET")?,
            files_dir: required("ROALER_FILES_DIR")?,
            default_rsshub_base_url: required("ROALER_DEFAULT_RSSHUB_BASE_URL")?,
            sync_interval_seconds: optional("ROALER_SYNC_INTERVAL_SECONDS")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(DEFAULT_SYNC_INTERVAL_SECONDS),
        })
    }
}

fn required(name: &str) -> AppResult<String> {
    env::var(name).map_err(|_| AppError::validation(format!("missing env {}", name)))
}

fn optional(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

