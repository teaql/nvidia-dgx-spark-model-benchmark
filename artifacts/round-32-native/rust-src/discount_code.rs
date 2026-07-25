// Auto-generated TeaQL Entity: DiscountCode
// Entity Index: 689
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountCode {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub discount_percentage: i64,
    pub valid_until: String,
}

impl DiscountCode {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
