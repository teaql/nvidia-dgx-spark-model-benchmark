// Auto-generated TeaQL Entity: CustomEntity194
// Entity Index: 75
// Source Module: module_12.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity194 {
    pub id: u64,
    pub name: String,
    pub metric: String,
    pub reading: String,
    pub recorded_at: String,
}

impl CustomEntity194 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
