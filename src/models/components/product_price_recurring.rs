use serde::{Deserialize, Serialize};

use super::{
    product_price_recurring_custom::ProductPriceRecurringCustom,
    product_price_recurring_fixed::ProductPriceRecurringFixed,
    product_price_recurring_free::ProductPriceRecurringFree,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductPriceRecurring {
    Free(ProductPriceRecurringFree),
    Fixed(ProductPriceRecurringFixed),
    Custom(ProductPriceRecurringCustom),
}
