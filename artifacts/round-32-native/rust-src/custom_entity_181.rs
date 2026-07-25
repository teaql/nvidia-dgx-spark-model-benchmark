// Auto-generated TeaQL Entity: CustomEntity181
// Entity Index: 62
// Source Module: module_12.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity181 {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub category: String,
    pub is_active: bool,
}

impl CustomEntity181 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
