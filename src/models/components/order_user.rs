use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderUser {
    pub id: String,
    pub email: String,
    pub public_name: String,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
}
