use crate::error::AppError;
use crate::search::client::SearchClient;

#[allow(dead_code)]
pub async fn ensure_index(client: &SearchClient) -> Result<(), AppError> {
    let http_client = client.inner();

    // Create index if it doesn't exist (ignore 409 conflict)
    let _ = http_client.create_index("entries", Some("id")).await;

    let index = http_client.index("entries");

    index
        .set_filterable_attributes(["pos", "domain", "register"])
        .await
        .map_err(|e| AppError::Search(format!("filterable attributes error: {e}")))?;

    index
        .set_searchable_attributes(["lemma_latin", "def_indonesian", "def_english"])
        .await
        .map_err(|e| AppError::Search(format!("searchable attributes error: {e}")))?;

    index
        .set_displayed_attributes([
            "id",
            "lemma_latin",
            "pos",
            "domain",
            "register",
            "def_indonesian",
            "def_english",
        ])
        .await
        .map_err(|e| AppError::Search(format!("displayed attributes error: {e}")))?;

    index
        .set_ranking_rules([
            "words",
            "typo",
            "proximity",
            "attribute",
            "sort",
            "exactness",
        ])
        .await
        .map_err(|e| AppError::Search(format!("ranking rules error: {e}")))?;

    Ok(())
}
