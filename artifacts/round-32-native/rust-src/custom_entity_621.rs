// Auto-generated TeaQL Entity: CustomEntity621
// Entity Index: 547
// Source Module: module_41.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity621 {
    pub id: u64,
    pub name: String,
    pub summary: String,
    pub details: String,
    pub published_at: String,
}

impl CustomEntity621 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
