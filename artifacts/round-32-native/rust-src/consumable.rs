// Auto-generated TeaQL Entity: Consumable
// Entity Index: 972
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumable {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub quantity: i64,
    pub unit_cost: String,
    pub reorder_level: i64,
}

impl Consumable {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
