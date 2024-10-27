use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    order_billing_reason::OrderBillingReason, order_product::OrderProduct,
    order_subscription::OrderSubscription, order_user::OrderUser, product_price::ProductPrice,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub metadata: HashMap<String, String>,
    pub amount: i64,
    pub tax_amount: i64,
    pub currency: String,
    pub billing_reason: OrderBillingReason,
    pub user_id: String,
    pub product_id: String,
    pub product_price_id: String,
    pub subscription_id: Option<String>,
    pub checkout_id: Option<String>,
    pub user: OrderUser,
    pub product: OrderProduct,
    pub product_price: ProductPrice,
    pub subscription: Option<OrderSubscription>,
}
