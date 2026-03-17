use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryEvent {
    pub id: Uuid,
    pub entry_id: Uuid,
    pub editor_id: Option<Uuid>,
    pub event_type: String,
    pub diff: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
