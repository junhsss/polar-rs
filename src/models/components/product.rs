use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{benefit_articles::BenefitArticles, benefit_base::BenefitBase, product_media_file_read::{ProductMediaFileRead, ProductMediaFileReadInput}, product_price::ProductPrice};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_recurring: bool,
    pub is_archived: bool,
    pub organization_id: String,
    pub prices: Vec<ProductPrice>,
    pub benefits: Vec<ProductBenefit>,
    pub medias: Vec<ProductMediaFileRead>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInput {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_recurring: bool,
    pub is_archived: bool,
    pub organization_id: String,
    pub prices: Vec<ProductPrice>,
    pub benefits: Vec<ProductBenefit>,
    pub medias: Vec<ProductMediaFileReadInput>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductBenefit {
    Base(BenefitBase),
    Articles(BenefitArticles),
}