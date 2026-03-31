use axum::{
    extract::{Path, State},
    middleware,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::jwt_middleware,
    error::AppError,
    models::{
        entry::{Attestation, Entry, Sense},
        etymology::Etymology,
        register::EntryRegister,
    },
};

use super::AppState;

// Helper function to log entry events
async fn log_entry_event(
    pool: &sqlx::PgPool,
    entry_id: Uuid,
    event_type: &str,
    diff: serde_json::Value,
    editor_id: Option<Uuid>,
) -> Result<(), AppError> {
    let event_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO entry_event (id, entry_id, editor_id, event_type, diff, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "#,
        event_id,
        entry_id,
        editor_id,
        event_type,
        diff
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(format!("event log error: {e}")))?;

    Ok(())
}

#[derive(Serialize)]
struct EntryResponse {
    entry: Entry,
    senses: Vec<Sense>,
    registers: Vec<EntryRegister>,
    etymologies: Vec<Etymology>,
}

#[derive(Deserialize)]
struct CreateEntryRequest {
    lemma_latin: String,
    pos: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct UpdateEntryRequest {
    pos: Option<String>,
    status: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/entries/:lemma", get(get_entry))
        .route("/entries/:id/attestations", get(get_attestations))
        .route(
            "/entries",
            post(create_entry).route_layer(middleware::from_fn(jwt_middleware)),
        )
        .route(
            "/entries/id/:id",
            put(update_entry).route_layer(middleware::from_fn(jwt_middleware)),
        )
}

async fn get_entry(
    State(state): State<AppState>,
    Path(lemma): Path<String>,
) -> Result<Json<EntryResponse>, AppError> {
    let entry = sqlx::query_as!(
        Entry,
        r#"
        SELECT id, lemma_latin, lemma_aksara, ipa, pos, root, status, created_at, updated_at
        FROM entry
        WHERE lemma_latin = $1
    "#,
        lemma
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("query error: {e}")))?
    .ok_or(AppError::NotFound)?;

    let senses = sqlx::query_as!(
        Sense,
        r#"
        SELECT id, entry_id as "entry_id!", sense_order, def_balinese, def_indonesian, def_english, domain, created_at
        FROM sense
        WHERE entry_id = $1
        ORDER BY sense_order
        "#,
        entry.id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("senses query error: {e}")))?;

    let registers = sqlx::query_as!(
        EntryRegister,
        r#"
        SELECT entry_id, level, dialect, equivalent_id
        FROM entry_register
        WHERE entry_id = $1
        "#,
        entry.id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("register query error: {e}")))?;

    let etymologies = sqlx::query_as!(
        Etymology,
        r#"
        SELECT id, entry_id as "entry_id!", proto_austronesian, proto_mp, sanskrit, kawi, old_balinese, loan_source, loan_form, notes, confidence
        FROM etymology
        WHERE entry_id = $1
        "#,
        entry.id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("etymology query error: {e}")))?;

    Ok(Json(EntryResponse {
        entry,
        senses,
        registers,
        etymologies,
    }))
}

#[derive(serde::Serialize)]
struct MeiliEntryDoc {
    id: Uuid,
    lemma_latin: String,
    pos: Option<String>,
    domain: Vec<String>,
    register: Vec<String>,
    def_indonesian: Option<String>,
    def_english: Option<String>,
}

async fn sync_entry_to_meili(state: &AppState, entry_id: Uuid) -> Result<(), AppError> {
    let row = sqlx::query!(
        r#"
        SELECT
            e.id,
            e.lemma_latin,
            e.pos,
            array_remove(array_agg(DISTINCT s.domain), NULL) AS domains,
            array_remove(array_agg(DISTINCT er.level), NULL) AS registers,
            (SELECT def_indonesian FROM sense WHERE entry_id = $1 ORDER BY sense_order LIMIT 1) AS def_indonesian,
            (SELECT def_english FROM sense WHERE entry_id = $1 ORDER BY sense_order LIMIT 1) AS def_english
        FROM entry e
        LEFT JOIN sense s ON s.entry_id = e.id
        LEFT JOIN entry_register er ON er.entry_id = e.id
        WHERE e.id = $1
        GROUP BY e.id, e.lemma_latin, e.pos
        "#,
        entry_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("meili fetch error: {e}")))?
    .ok_or(AppError::NotFound)?;

    let domains: Vec<String> = row.domains.unwrap_or_default();
    let registers: Vec<String> = row.registers.unwrap_or_default();

    let doc = MeiliEntryDoc {
        id: row.id,
        lemma_latin: row.lemma_latin,
        pos: row.pos,
        domain: domains,
        register: registers,
        def_indonesian: row.def_indonesian,
        def_english: row.def_english,
    };

    let index = state.search.inner().index("entries");
    index
        .add_or_replace(&[doc], Some("id"))
        .await
        .map_err(|e| AppError::Search(format!("meili sync error: {e}")))?;

    Ok(())
}

async fn create_entry(
    State(state): State<AppState>,
    Json(payload): Json<CreateEntryRequest>,
) -> Result<Json<EntryResponse>, AppError> {
    if payload.lemma_latin.trim().is_empty() {
        return Err(AppError::Validation("lemma_latin required".into()));
    }

    let entry_id = Uuid::new_v4();

    let rec = sqlx::query_as!(
        Entry,
        r#"
        INSERT INTO entry (id, lemma_latin, pos, status)
        VALUES ($1, $2, $3, 'draft')
        RETURNING id, lemma_latin, lemma_aksara, ipa, pos, root, status, created_at, updated_at
    "#,
        entry_id,
        payload.lemma_latin.trim(),
        payload.pos
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("insert error: {e}")))?;

    // Log creation event
    let diff = serde_json::json!({
        "lemma_latin": payload.lemma_latin.trim(),
        "pos": payload.pos,
        "status": "draft",
    });

    log_entry_event(&state.pool, entry_id, "create", diff, None).await?;

    Ok(Json(EntryResponse {
        entry: rec,
        senses: vec![],
        registers: vec![],
        etymologies: vec![],
    }))
}

async fn update_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateEntryRequest>,
) -> Result<Json<EntryResponse>, AppError> {
    // Fetch current status for publish detection
    let current = sqlx::query!(
        r#"
        SELECT status FROM entry WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("status fetch error: {e}")))?;

    if current.is_none() {
        return Err(AppError::NotFound);
    }

    let prev_status = current.unwrap().status;

    // Start transaction
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|e| AppError::Database(format!("transaction error: {e}")))?;

    // Update entry with provided fields
    let entry_row = sqlx::query!(
        r#"
        UPDATE entry
        SET pos = COALESCE($2, pos),
            status = COALESCE($3, status),
            updated_at = now()
        WHERE id = $1
        RETURNING id, lemma_latin, lemma_aksara, ipa, pos, root, status, created_at, updated_at
        "#,
        id,
        payload.pos.as_deref(),
        payload.status.as_deref()
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| AppError::Database(format!("update error: {e}")))?
    .ok_or(AppError::NotFound)?;

    let entry = Entry {
        id: entry_row.id,
        lemma_latin: entry_row.lemma_latin,
        lemma_aksara: entry_row.lemma_aksara,
        ipa: entry_row.ipa,
        pos: entry_row.pos,
        root: entry_row.root,
        status: entry_row.status,
        created_at: entry_row.created_at,
        updated_at: entry_row.updated_at,
    };

    // Log event
    let diff = serde_json::json!({
        "pos": payload.pos,
        "status": payload.status,
        "previous_status": prev_status,
    });

    log_entry_event(&state.pool, id, "update", diff, None).await?;

    tx.commit()
        .await
        .map_err(|e| AppError::Database(format!("commit error: {e}")))?;

    // If published now, push to Meilisearch
    if payload
        .status
        .as_deref()
        .map(|s| s == "published")
        .unwrap_or(false)
        && prev_status != "published"
    {
        sync_entry_to_meili(&state, id).await?;
    }

    // Fetch full entry with related data
    let senses = sqlx::query_as!(
        Sense,
        r#"
        SELECT id, entry_id as "entry_id!", sense_order, def_balinese, def_indonesian, def_english, domain, created_at
        FROM sense
        WHERE entry_id = $1
        ORDER BY sense_order
        "#,
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("sense query error: {e}")))?;

    let registers = sqlx::query_as!(
        EntryRegister,
        r#"
        SELECT entry_id as "entry_id!", level, dialect, equivalent_id
        FROM entry_register
        WHERE entry_id = $1
        "#,
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("register query error: {e}")))?;

    let etymologies = sqlx::query_as!(
        Etymology,
        r#"
        SELECT id, entry_id as "entry_id!", proto_austronesian, proto_mp, sanskrit, kawi, old_balinese, loan_source, loan_form, notes, confidence
        FROM etymology
        WHERE entry_id = $1
        "#,
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("etymology query error: {e}")))?;

    Ok(Json(EntryResponse {
        entry,
        senses,
        registers,
        etymologies,
    }))
}

async fn get_attestations(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<Attestation>>, AppError> {
    // Fetch attestations by entry id via senses
    let attestations = sqlx::query_as!(
        Attestation,
        r#"
        SELECT a.id, a.sense_id as "sense_id!", a.corpus_id as "corpus_id!", a.quote_aksara, a.quote_latin, a.quote_trans_id, a.quote_trans_en, a.confidence::float8 as confidence, a.source_rank, a.created_at
        FROM attestation a
        JOIN sense s ON a.sense_id = s.id
        WHERE s.entry_id = $1
        ORDER BY a.created_at DESC
        "#,
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("attestation query error: {e}")))?;

    Ok(Json(attestations))
}
