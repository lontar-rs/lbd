use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppError, models::event::EntryEvent};

use super::AppState;

#[derive(Deserialize)]
pub struct EventQuery {
    pub entry_id: Option<Uuid>,
    pub event_type: Option<String>,
    pub limit: Option<i64>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/events", get(get_events))
        .route("/events/entry/:entry_id", get(get_entry_events))
}

async fn get_events(
    State(state): State<AppState>,
    Query(params): Query<EventQuery>,
) -> Result<Json<Vec<EntryEvent>>, AppError> {
    let events = if let Some(entry_id) = params.entry_id {
        if let Some(event_type) = params.event_type {
            sqlx::query_as!(
                EntryEvent,
                r#"
                SELECT id, entry_id as "entry_id!", editor_id, event_type, diff, created_at
                FROM entry_event
                WHERE entry_id = $1 AND event_type = $2
                ORDER BY created_at DESC
                "#,
                entry_id,
                event_type
            )
            .fetch_all(&state.pool)
            .await
            .map_err(|e| AppError::Database(format!("events query error: {e}")))?
        } else {
            sqlx::query_as!(
                EntryEvent,
                r#"
                SELECT id, entry_id as "entry_id!", editor_id, event_type, diff, created_at
                FROM entry_event
                WHERE entry_id = $1
                ORDER BY created_at DESC
                "#,
                entry_id
            )
            .fetch_all(&state.pool)
            .await
            .map_err(|e| AppError::Database(format!("events query error: {e}")))?
        }
    } else if let Some(event_type) = params.event_type {
        sqlx::query_as!(
            EntryEvent,
            r#"
            SELECT id, entry_id as "entry_id!", editor_id, event_type, diff, created_at
            FROM entry_event
            WHERE event_type = $1
            ORDER BY created_at DESC
            "#,
            event_type
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| AppError::Database(format!("events query error: {e}")))?
    } else {
        sqlx::query_as!(
            EntryEvent,
            r#"
            SELECT id, entry_id as "entry_id!", editor_id, event_type, diff, created_at
            FROM entry_event
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| AppError::Database(format!("events query error: {e}")))?
    };

    // Apply limit if specified
    let events = if let Some(limit) = params.limit {
        events.into_iter().take(limit as usize).collect()
    } else {
        events
    };

    Ok(Json(events))
}

async fn get_entry_events(
    State(state): State<AppState>,
    Path(entry_id): Path<Uuid>,
) -> Result<Json<Vec<EntryEvent>>, AppError> {
    let events = sqlx::query_as!(
        EntryEvent,
        r#"
        SELECT id, entry_id as "entry_id!", editor_id, event_type, diff, created_at
        FROM entry_event
        WHERE entry_id = $1
        ORDER BY created_at DESC
        "#,
        entry_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("entry events query error: {e}")))?;

    Ok(Json(events))
}
