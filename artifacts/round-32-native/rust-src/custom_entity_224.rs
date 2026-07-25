// Auto-generated TeaQL Entity: CustomEntity224
// Entity Index: 105
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity224 {
    pub id: u64,
    pub name: String,
    pub summary: String,
    pub category: String,
    pub priority_level: i64,
    pub created_at: String,
}

impl CustomEntity224 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
