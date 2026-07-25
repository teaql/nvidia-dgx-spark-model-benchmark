// Auto-generated TeaQL Entity: CustomEntity449
// Entity Index: 345
// Source Module: module_29.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity449 {
    pub id: u64,
    pub name: String,
    pub metric: String,
    pub reading: String,
    pub recorded_at: String,
}

impl CustomEntity449 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
