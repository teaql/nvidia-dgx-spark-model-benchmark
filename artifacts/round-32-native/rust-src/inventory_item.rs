// Auto-generated TeaQL Entity: InventoryItem
// Entity Index: 19
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub quantity: i64,
}

impl InventoryItem {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
