use serde::{Deserialize, Serialize};

use super::{
    product_price_one_time_custom::ProductPriceOneTimeCustom,
    product_price_one_time_fixed::ProductPriceOneTimeFixed,
    product_price_one_time_free::ProductPriceOneTimeFree,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductPriceOneTime {
    Free(ProductPriceOneTimeFree),
    Fixed(ProductPriceOneTimeFixed),
    Custom(ProductPriceOneTimeCustom),
}
