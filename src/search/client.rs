use meilisearch_sdk::client::Client;

use crate::error::AppError;

#[derive(Clone)]
pub struct SearchClient {
    #[allow(dead_code)]
    inner: Client,
}

impl SearchClient {
    pub fn new(inner: Client) -> Self {
        Self { inner }
    }

    pub async fn search(
        &self,
        index: &str,
        query: &str,
        filter: Option<&str>,
    ) -> Result<serde_json::Value, AppError> {
        let _ = (index, query, filter); // placeholders until search wiring is completed
                                        // TODO: wire real Meilisearch query when index schema is ready
        Ok(serde_json::json!({
            "hits": [],
            "estimated_total_hits": 0,
            "query": query,
        }))
    }
}

pub fn init(url: &str, key: &str) -> Result<SearchClient, AppError> {
    let client = Client::new(url, Some(key)).map_err(|e| AppError::Search(e.to_string()))?;
    Ok(SearchClient::new(client))
}
