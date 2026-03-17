use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Etymology {
    pub id: Uuid,
    pub entry_id: Uuid,
    pub proto_austronesian: Option<String>,
    pub proto_mp: Option<String>,
    pub sanskrit: Option<String>,
    pub kawi: Option<String>,
    pub old_balinese: Option<String>,
    pub loan_source: Option<String>,
    pub loan_form: Option<String>,
    pub notes: Option<String>,
    pub confidence: String,
}
