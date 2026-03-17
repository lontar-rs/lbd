use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Corpus {
    pub id: Uuid,
    pub title: String,
    pub r#type: String,
    pub date_min: Option<i32>,
    pub date_max: Option<i32>,
    pub date_cert: Option<String>,
    pub period: String,
    pub script: String,
    pub location: Option<String>,
    pub call_number: Option<String>,
    pub dig_status: String,
    pub license: Option<String>,
    pub created_at: DateTime<Utc>,
}
