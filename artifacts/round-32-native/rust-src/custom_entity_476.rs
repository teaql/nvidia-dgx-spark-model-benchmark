// Auto-generated TeaQL Entity: CustomEntity476
// Entity Index: 387
// Source Module: module_31.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity476 {
    pub id: u64,
    pub name: String,
    pub level: String,
    pub weight: String,
    pub updated_at: String,
}

impl CustomEntity476 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
