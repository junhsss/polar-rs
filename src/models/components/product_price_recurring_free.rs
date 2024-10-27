use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::subscription_recurring_interval::SubscriptionRecurringInterval;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceRecurringFree {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub amount_type: Option<String>,
    pub is_archived: bool,
    pub product_id: String,
    #[serde(rename = "type")]
    pub price_type: Option<String>,
    pub recurring_interval: SubscriptionRecurringInterval,
}
