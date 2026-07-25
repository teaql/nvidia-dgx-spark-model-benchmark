// Auto-generated TeaQL Entity: CustomEntity306
// Entity Index: 202
// Source Module: module_20.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity306 {
    pub id: u64,
    pub name: String,
    pub summary: String,
    pub details: String,
    pub published_at: String,
}

impl CustomEntity306 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
