use serde::{Deserialize, Serialize};

use super::order::Order;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookOrderCreatedPayload {
    #[serde(rename = "type")]
    pub webhook_type: Option<String>,
    pub data: Order,
}
