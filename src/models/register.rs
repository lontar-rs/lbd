use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryRegister {
    pub entry_id: Uuid,
    pub level: String,
    pub dialect: Option<String>,
    pub equivalent_id: Option<Uuid>,
}
