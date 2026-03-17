use axum::{
    extract::{Path, State},
    middleware,
    routing::{get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::jwt_middleware, error::AppError, models::entry::Entry};

use super::AppState;

#[derive(Serialize)]
struct EntryResponse {
    entry: Entry,
    senses: Vec<SenseDto>,
    registers: Vec<RegisterDto>,
    etymologies: Vec<EtymologyDto>,
}

#[derive(Deserialize)]
struct CreateEntryRequest {
    lemma_latin: String,
    pos: Option<String>,
}

#[derive(Serialize)]
struct SenseDto {
    id: Uuid,
    sense_order: i32,
    def_balinese: Option<String>,
    def_indonesian: String,
    def_english: Option<String>,
    domain: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Serialize)]
struct RegisterDto {
    level: String,
    dialect: Option<String>,
    equivalent_id: Option<Uuid>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct UpdateEntryRequest {
    pos: Option<String>,
    status: Option<String>,
}

#[derive(Serialize)]
struct EtymologyDto {
    id: Uuid,
    proto_austronesian: Option<String>,
    proto_mp: Option<String>,
    sanskrit: Option<String>,
    kawi: Option<String>,
    old_balinese: Option<String>,
    loan_source: Option<String>,
    loan_form: Option<String>,
    notes: Option<String>,
    confidence: String,
}

#[derive(Serialize)]
struct AttestationDto {
    id: Uuid,
    sense_id: Uuid,
    corpus_id: Uuid,
    quote_aksara: Option<String>,
    quote_latin: Option<String>,
    quote_trans_id: Option<String>,
    quote_trans_en: Option<String>,
    confidence: Option<f32>,
    source_rank: i32,
    created_at: DateTime<Utc>,
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

    let senses = sqlx::query!(
        r#"
        SELECT id, sense_order, def_balinese, def_indonesian, def_english, domain, created_at
        FROM sense
        WHERE entry_id = $1
        ORDER BY sense_order
        "#,
        entry.id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("senses query error: {e}")))?
    .into_iter()
    .map(|row| SenseDto {
        id: row.id,
        sense_order: row.sense_order,
        def_balinese: row.def_balinese,
        def_indonesian: row.def_indonesian,
        def_english: row.def_english,
        domain: row.domain,
        created_at: row.created_at,
    })
    .collect();

    let registers = sqlx::query!(
        r#"
        SELECT level, dialect, equivalent_id
        FROM entry_register
        WHERE entry_id = $1
        "#,
        entry.id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("register query error: {e}")))?
    .into_iter()
    .map(|row| RegisterDto {
        level: row.level,
        dialect: row.dialect,
        equivalent_id: row.equivalent_id,
    })
    .collect();

    let etymologies = sqlx::query!(
        r#"
        SELECT id, proto_austronesian, proto_mp, sanskrit, kawi, old_balinese, loan_source, loan_form, notes, confidence
        FROM etymology
        WHERE entry_id = $1
        "#,
        entry.id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("etymology query error: {e}")))?
    .into_iter()
    .map(|row| EtymologyDto {
        id: row.id,
        proto_austronesian: row.proto_austronesian,
        proto_mp: row.proto_mp,
        sanskrit: row.sanskrit,
        kawi: row.kawi,
        old_balinese: row.old_balinese,
        loan_source: row.loan_source,
        loan_form: row.loan_form,
        notes: row.notes,
        confidence: row.confidence,
    })
    .collect();

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

    let rec = sqlx::query_as!(
        Entry,
        r#"
        INSERT INTO entry (id, lemma_latin, pos, status)
        VALUES ($1, $2, $3, 'draft')
        RETURNING id, lemma_latin, lemma_aksara, ipa, pos, root, status, created_at, updated_at
    "#,
        Uuid::new_v4(),
        payload.lemma_latin.trim(),
        payload.pos
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("insert error: {e}")))?;

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
    let event_id = Uuid::new_v4();
    let diff = serde_json::json!({
        "pos": payload.pos,
        "status": payload.status,
    });

    sqlx::query!(
        r#"
        INSERT INTO entry_event (id, entry_id, event_type, diff)
        VALUES ($1, $2, 'update', $3)
        "#,
        event_id,
        id,
        diff
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Database(format!("event log error: {e}")))?;

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
    let senses = sqlx::query!(
        r#"
        SELECT id, sense_order, def_balinese, def_indonesian, def_english, domain, created_at
        FROM sense
        WHERE entry_id = $1
        ORDER BY sense_order
        "#,
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("sense query error: {e}")))?
    .into_iter()
    .map(|row| SenseDto {
        id: row.id,
        sense_order: row.sense_order,
        def_balinese: row.def_balinese,
        def_indonesian: row.def_indonesian,
        def_english: row.def_english,
        domain: row.domain,
        created_at: row.created_at,
    })
    .collect();

    let registers = sqlx::query_as::<_, (String, Option<String>, Option<Uuid>)>(
        r#"
        SELECT level, dialect, equivalent_id
        FROM entry_register
        WHERE entry_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("register query error: {e}")))?
    .into_iter()
    .map(|(level, dialect, equivalent_id)| RegisterDto {
        level,
        dialect,
        equivalent_id,
    })
    .collect();

    let etymologies = sqlx::query!(
        r#"
        SELECT id, proto_austronesian, proto_mp, sanskrit, kawi, old_balinese, loan_source, loan_form, notes, confidence
        FROM etymology
        WHERE entry_id = $1
        "#,
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("etymology query error: {e}")))?
    .into_iter()
    .map(|row| EtymologyDto {
        id: row.id,
        proto_austronesian: row.proto_austronesian,
        proto_mp: row.proto_mp,
        sanskrit: row.sanskrit,
        kawi: row.kawi,
        old_balinese: row.old_balinese,
        loan_source: row.loan_source,
        loan_form: row.loan_form,
        notes: row.notes,
        confidence: row.confidence,
    })
    .collect();

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
) -> Result<Json<Vec<AttestationDto>>, AppError> {
    // Fetch attestations by entry id via senses
    let rows = sqlx::query!(
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

    let attns = rows
        .into_iter()
        .map(|row| AttestationDto {
            id: row.id,
            sense_id: row.sense_id,
            corpus_id: row.corpus_id,
            quote_aksara: row.quote_aksara,
            quote_latin: row.quote_latin,
            quote_trans_id: row.quote_trans_id,
            quote_trans_en: row.quote_trans_en,
            confidence: row.confidence.map(|c| c as f32),
            source_rank: row.source_rank,
            created_at: row.created_at,
        })
        .collect();

    Ok(Json(attns))
}
