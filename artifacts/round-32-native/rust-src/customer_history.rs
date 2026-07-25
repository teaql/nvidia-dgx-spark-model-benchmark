// Auto-generated TeaQL Entity: CustomerHistory
// Entity Index: 359
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerHistory {
    pub id: u64,
    pub name: String,
    pub purchase_volume: i64,
    pub lifetime_value: f64,
}

impl CustomerHistory {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
