use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub lemma_latin: String,
    pub lemma_aksara: Option<String>,
    pub ipa: Option<String>,
    pub pos: Option<String>,
    pub root: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NewEntry {
    pub lemma_latin: String,
    pub pos: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sense {
    pub id: Uuid,
    pub entry_id: Uuid,
    pub sense_order: i32,
    pub def_balinese: Option<String>,
    pub def_indonesian: String,
    pub def_english: Option<String>,
    pub domain: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attestation {
    pub id: Uuid,
    pub sense_id: Uuid,
    pub corpus_id: Uuid,
    pub quote_aksara: Option<String>,
    pub quote_latin: Option<String>,
    pub quote_trans_id: Option<String>,
    pub quote_trans_en: Option<String>,
    pub confidence: Option<f32>,
    pub source_rank: i32,
    pub created_at: DateTime<Utc>,
}
