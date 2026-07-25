// Auto-generated TeaQL Entity: CustomEntity546
// Entity Index: 457
// Source Module: module_36.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity546 {
    pub id: u64,
    pub name: String,
    pub summary: String,
    pub details: String,
    pub published_at: String,
}

impl CustomEntity546 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
