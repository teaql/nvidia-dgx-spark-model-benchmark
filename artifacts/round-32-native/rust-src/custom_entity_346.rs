// Auto-generated TeaQL Entity: CustomEntity346
// Entity Index: 242
// Source Module: module_23.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity346 {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub category: String,
    pub is_active: bool,
}

impl CustomEntity346 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
