use meilisearch_sdk::client::Client;
use meilisearch_sdk::search::SearchQuery;

use crate::error::AppError;

#[derive(Clone)]
pub struct SearchClient {
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
        let index_handle = self.inner.index(index);
        let mut builder = SearchQuery::new(&index_handle);
        builder.with_query(query);
        if let Some(f) = filter {
            builder.with_filter(f);
        }

        let res = builder
            .execute::<serde_json::Value>()
            .await
            .map_err(|e| AppError::Search(e.to_string()))?;

        let hits: Vec<serde_json::Value> = res.hits.into_iter().map(|h| h.result).collect();

        Ok(serde_json::json!({
            "hits": hits,
            "offset": res.offset,
            "limit": res.limit,
            "estimated_total_hits": res.estimated_total_hits,
            "processing_time_ms": res.processing_time_ms,
            "query": res.query,
            "facet_distribution": res.facet_distribution,
        }))
    }
}

pub fn init(url: &str, key: &str) -> Result<SearchClient, AppError> {
    let client = Client::new(url, Some(key)).map_err(|e| AppError::Search(e.to_string()))?;
    Ok(SearchClient::new(client))
}
