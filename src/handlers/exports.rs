use std::path::PathBuf;

use axum::{
    extract::State,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::Utc;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, Connection, Executor,
};
use tokio::fs;
use tokio::time::{interval, Duration};
use tracing::error;

use crate::error::AppError;

use super::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/exports/latest", get(download_latest_snapshot))
}

async fn download_latest_snapshot(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let path = generate_snapshot(&state).await?;
    let bytes = fs::read(&path)
        .await
        .map_err(|e| AppError::Database(format!("read snapshot error: {e}")))?;

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("snapshot.sqlite");
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
            .unwrap_or(HeaderValue::from_static("attachment")),
    );

    Ok((StatusCode::OK, headers, bytes))
}

async fn generate_snapshot(state: &AppState) -> Result<PathBuf, AppError> {
    fs::create_dir_all(&state.export_dir)
        .await
        .map_err(|e| AppError::Database(format!("export dir error: {e}")))?;

    let filename = format!("lbd_snapshot_{}.sqlite", Utc::now().format("%Y%m%d_%H%M%S"));
    let path = PathBuf::from(&state.export_dir).join(filename);

    // Build SQLite DB
    let mut sqlite = SqliteConnectOptions::new()
        .filename(&path)
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true)
        .connect()
        .await
        .map_err(|e| AppError::Database(format!("sqlite open error: {e}")))?;

    sqlite
        .execute(
            r#"
        CREATE TABLE IF NOT EXISTS entry (
            id TEXT PRIMARY KEY,
            lemma_latin TEXT NOT NULL,
            pos TEXT,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS sense (
            id TEXT PRIMARY KEY,
            entry_id TEXT NOT NULL,
            sense_order INTEGER NOT NULL,
            def_indonesian TEXT,
            def_english TEXT,
            domain TEXT,
            created_at TEXT NOT NULL
        );
        "#,
        )
        .await
        .map_err(|e| AppError::Database(format!("sqlite schema error: {e}")))?;

    // Export entries
    let entries = sqlx::query!(
        r#"
        SELECT id, lemma_latin, pos, status, created_at, updated_at
        FROM entry
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("entries fetch error: {e}")))?;

    let mut tx = sqlite
        .begin()
        .await
        .map_err(|e| AppError::Database(format!("sqlite tx error: {e}")))?;

    for row in entries {
        sqlx::query(
            r#"INSERT INTO entry (id, lemma_latin, pos, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(row.id.to_string())
        .bind(row.lemma_latin)
        .bind(row.pos)
        .bind(row.status)
        .bind(row.created_at.to_rfc3339())
        .bind(row.updated_at.to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(format!("sqlite insert entry error: {e}")))?;
    }

    // Export senses
    let senses = sqlx::query!(
        r#"
        SELECT id, entry_id, sense_order, def_indonesian, def_english, domain, created_at
        FROM sense
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("senses fetch error: {e}")))?;

    for row in senses {
        sqlx::query(
            r#"INSERT INTO sense (id, entry_id, sense_order, def_indonesian, def_english, domain, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(row.id.to_string())
        .bind(row.entry_id)
        .bind(row.sense_order)
        .bind(row.def_indonesian)
        .bind(row.def_english)
        .bind(row.domain)
        .bind(row.created_at.to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(format!("sqlite insert sense error: {e}")))?;
    }

    tx.commit()
        .await
        .map_err(|e| AppError::Database(format!("sqlite commit error: {e}")))?;

    Ok(path)
}

pub async fn start_export_scheduler(state: AppState) {
    // Weekly interval
    let mut ticker = interval(Duration::from_secs(7 * 24 * 60 * 60));
    loop {
        ticker.tick().await;
        if let Err(e) = generate_snapshot(&state).await {
            error!(?e, "scheduled export failed");
        }
    }
}
