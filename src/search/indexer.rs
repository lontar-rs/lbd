use crate::error::AppError;
use crate::search::client::SearchClient;

#[allow(dead_code)]
pub async fn ensure_index(client: &SearchClient) -> Result<(), AppError> {
    let _ = client
        .search("entries", "", None)
        .await
        .map_err(|e| AppError::Search(format!("index check failed: {e}")))?;
    Ok(())
}
