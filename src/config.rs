use std::env;

use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub meilisearch_url: String,
    pub meilisearch_key: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("DATABASE_URL not set".into()))?;
        let meilisearch_url = env::var("MEILISEARCH_URL")
            .map_err(|_| AppError::Config("MEILISEARCH_URL not set".into()))?;
        let meilisearch_key = env::var("MEILISEARCH_KEY").unwrap_or_default();
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse()
            .map_err(|_| AppError::Config("SERVER_PORT must be a number".into()))?;

        Ok(Self {
            database_url,
            meilisearch_url,
            meilisearch_key,
            host,
            port,
        })
    }
}
