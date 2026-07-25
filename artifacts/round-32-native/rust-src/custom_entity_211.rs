// Auto-generated TeaQL Entity: CustomEntity211
// Entity Index: 92
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity211 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub category: String,
    pub priority: i64,
    pub active: bool,
}

impl CustomEntity211 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
