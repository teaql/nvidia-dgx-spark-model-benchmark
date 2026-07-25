// Auto-generated TeaQL Entity: InsuranceAddon
// Entity Index: 681
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceAddon {
    pub id: u64,
    pub name: String,
    pub coverage_limit: f64,
    pub premium_amount: i64,
}

impl InsuranceAddon {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
