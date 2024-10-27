use serde::{Deserialize, Serialize};

use super::{
    product_price_one_time::ProductPriceOneTime, product_price_recurring::ProductPriceRecurring,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductPrice {
    Recurring(ProductPriceRecurring),
    OneTime(ProductPriceOneTime),
}
