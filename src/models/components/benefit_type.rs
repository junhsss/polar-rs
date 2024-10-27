use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenefitType {
    Custom,
    Articles,
    Ads,
    Discord,
    GithubRepository,
    Downloadables,
    LicenseKeys,
}
