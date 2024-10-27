use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::benefit_articles_properties::BenefitArticlesProperties;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenefitArticles {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    #[serde(rename = "type")]
    pub benefit_type: Option<String>,
    pub description: String,
    pub selectable: bool,
    pub deletable: bool,
    pub organization_id: String,
    pub properties: BenefitArticlesProperties,
}
