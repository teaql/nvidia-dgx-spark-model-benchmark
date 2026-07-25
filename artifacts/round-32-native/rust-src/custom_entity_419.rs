// Auto-generated TeaQL Entity: CustomEntity419
// Entity Index: 315
// Source Module: module_27.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity419 {
    pub id: u64,
    pub name: String,
    pub metric: String,
    pub reading: String,
    pub recorded_at: String,
}

impl CustomEntity419 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
