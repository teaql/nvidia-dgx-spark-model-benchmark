// Auto-generated TeaQL Entity: PackingMaterial
// Entity Index: 680
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackingMaterial {
    pub id: u64,
    pub name: String,
    pub item_name: String,
    pub price_per_unit: f64,
    pub stock_quantity: i64,
}

impl PackingMaterial {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
