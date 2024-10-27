use serde::{Deserialize, Serialize};

use super::subscription::SubscriptionInput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscriptionCreatedPayload {
    #[serde(rename = "type")]
    pub webhook_type: Option<String>,
    pub data: SubscriptionInput,
}
