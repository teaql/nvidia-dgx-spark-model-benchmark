// Auto-generated TeaQL Entity: CustomEntity331
// Entity Index: 227
// Source Module: module_22.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity331 {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub category: String,
    pub is_active: bool,
}

impl CustomEntity331 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
