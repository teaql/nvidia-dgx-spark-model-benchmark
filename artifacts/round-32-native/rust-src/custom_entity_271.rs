// Auto-generated TeaQL Entity: CustomEntity271
// Entity Index: 152
// Source Module: module_18.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity271 {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub category: String,
    pub is_active: bool,
}

impl CustomEntity271 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
