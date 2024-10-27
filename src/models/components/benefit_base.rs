use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::benefit_type::BenefitType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenefitBase {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    #[serde(rename = "type")]
    pub benefit_type: BenefitType,
    pub description: String,
    pub selectable: bool,
    pub deletable: bool,
    pub organization_id: String,
}
