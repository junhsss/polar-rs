use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{product::ProductInput, product_price_recurring::ProductPriceRecurring, subscription_recurring_interval::SubscriptionRecurringInterval, subscription_status::SubscriptionStatus, subscription_user::SubscriptionUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionInput {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub recurring_interval: SubscriptionRecurringInterval,
    pub status: SubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub user_id: String,
    pub product_id: String,
    pub price_id: String,
    pub checkout_id: Option<String>,
    pub metadata: HashMap<String, String>,
    pub user: SubscriptionUser,
    pub product: ProductInput,
    pub price: ProductPriceRecurring,
}