// Auto-generated TeaQL Entity: CustomEntity846
// Entity Index: 787
// Source Module: module_56.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity846 {
    pub id: u64,
    pub name: String,
    pub summary: String,
    pub details: String,
    pub published_at: String,
}

impl CustomEntity846 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
