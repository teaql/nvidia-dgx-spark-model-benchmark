// Auto-generated TeaQL Entity: CustomEntity220
// Entity Index: 101
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity220 {
    pub id: u64,
    pub name: String,
    pub identifier: String,
    pub category: String,
    pub weight: String,
    pub status: String,
}

impl CustomEntity220 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
