use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceOneTimeCustom {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub amount_type: Option<String>,
    pub is_archived: bool,
    pub product_id: String,
    pub price_currency: String,
    pub minimum_amount: Option<i64>,
    pub maximum_amount: Option<i64>,
    pub preset_amount: Option<i64>,
    #[serde(rename = "type")]
    pub price_type: Option<String>,
}
