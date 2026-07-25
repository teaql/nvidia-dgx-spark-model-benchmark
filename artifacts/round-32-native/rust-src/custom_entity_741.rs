// Auto-generated TeaQL Entity: CustomEntity741
// Entity Index: 667
// Source Module: module_49.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity741 {
    pub id: u64,
    pub name: String,
    pub summary: String,
    pub details: String,
    pub published_at: String,
}

impl CustomEntity741 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
