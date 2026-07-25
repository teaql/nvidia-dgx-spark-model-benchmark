// Auto-generated TeaQL Entity: CustomEntity495
// Entity Index: 406
// Source Module: module_33.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity495 {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

impl CustomEntity495 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
