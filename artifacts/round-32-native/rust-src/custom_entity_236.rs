// Auto-generated TeaQL Entity: CustomEntity236
// Entity Index: 117
// Source Module: module_15.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity236 {
    pub id: u64,
    pub name: String,
    pub level: String,
    pub weight: String,
    pub updated_at: String,
}

impl CustomEntity236 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
