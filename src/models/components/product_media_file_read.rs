use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductMediaFileRead {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<DateTime<Utc>>,
    pub version: Option<String>,
    pub service: Option<String>,
    pub is_uploaded: bool,
    pub created_at: DateTime<Utc>,
    pub size_readable: String,
    pub public_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductMediaFileReadInput {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<DateTime<Utc>>,
    pub version: Option<String>,
    pub service: Option<String>, 
    pub is_uploaded: bool,
    pub created_at: DateTime<Utc>,
}
