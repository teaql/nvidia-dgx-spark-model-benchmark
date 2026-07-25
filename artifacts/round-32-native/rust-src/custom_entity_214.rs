// Auto-generated TeaQL Entity: CustomEntity214
// Entity Index: 95
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity214 {
    pub id: u64,
    pub name: String,
    pub item_code: String,
    pub quantity: i64,
    pub weight: String,
    pub status: String,
}

impl CustomEntity214 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
