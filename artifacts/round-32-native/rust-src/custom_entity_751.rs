// Auto-generated TeaQL Entity: CustomEntity751
// Entity Index: 692
// Source Module: module_50.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity751 {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub category: String,
    pub is_active: bool,
}

impl CustomEntity751 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
