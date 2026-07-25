// Auto-generated TeaQL Entity: CustomEntity255
// Entity Index: 136
// Source Module: module_17.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity255 {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

impl CustomEntity255 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
