// Auto-generated TeaQL Entity: CustomEntity494
// Entity Index: 405
// Source Module: module_32.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity494 {
    pub id: u64,
    pub name: String,
    pub metric: String,
    pub reading: String,
    pub recorded_at: String,
}

impl CustomEntity494 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
