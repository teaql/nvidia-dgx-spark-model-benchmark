// Auto-generated TeaQL Entity: CustomEntity731
// Entity Index: 657
// Source Module: module_48.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity731 {
    pub id: u64,
    pub name: String,
    pub level: String,
    pub weight: String,
    pub updated_at: String,
}

impl CustomEntity731 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
