// Auto-generated TeaQL Entity: CustomEntity824
// Entity Index: 765
// Source Module: module_54.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity824 {
    pub id: u64,
    pub name: String,
    pub metric: String,
    pub reading: String,
    pub recorded_at: String,
}

impl CustomEntity824 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
