use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{error::AppError, models::corpus::Corpus};

use super::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/corpus/:id", get(get_corpus))
}

async fn get_corpus(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Corpus>, AppError> {
    let rec = sqlx::query_as!(Corpus, r#"
        SELECT id, title, type AS "type!", date_min, date_max, date_cert, period, script, location, call_number, dig_status, license, created_at
        FROM corpus
        WHERE id = $1
    "#, id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| AppError::Database(format!("query error: {e}")))?;

    let Some(row) = rec else {
        return Err(AppError::NotFound);
    };
    Ok(Json(row))
}
