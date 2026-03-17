use meilisearch_sdk::indexes::Index;

use crate::error::AppError;
use crate::search::client::SearchClient;

#[allow(dead_code)]
pub async fn ensure_index(client: &SearchClient) -> Result<(), AppError> {
    let _index: Index = client.inner().index("entries");

    // Try to create index (ignore error if it already exists)
    let _ = client.inner().create_index("entries", Some("id")).await;

    Ok(())
}
