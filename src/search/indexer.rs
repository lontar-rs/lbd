use crate::error::AppError;
use crate::search::client::SearchClient;

#[allow(dead_code)]
pub async fn ensure_index(client: &SearchClient) -> Result<(), AppError> {
    let http_client = client.inner();

    // Create index if it doesn't exist (ignore 409 conflict)
    let _ = http_client.create_index("entries", Some("id")).await;

    // TODO: Configure index settings (filterableAttributes, searchableAttributes, etc.)
    // The Meilisearch SDK has trait bound issues with attribute collections.
    // For now, index is created with default settings.
    // Settings can be configured via:
    // - Direct HTTP PATCH to /indexes/entries/settings
    // - Or wait for SDK improvements in future versions

    Ok(())
}
