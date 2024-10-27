use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderProduct {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_recurring: bool,
    pub is_archived: bool,
    pub organization_id: String,
}
