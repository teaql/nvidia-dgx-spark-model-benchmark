// Auto-generated TeaQL Entity: CustomEntity270
// Entity Index: 151
// Source Module: module_18.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity270 {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

impl CustomEntity270 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
