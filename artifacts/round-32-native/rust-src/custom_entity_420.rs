// Auto-generated TeaQL Entity: CustomEntity420
// Entity Index: 316
// Source Module: module_28.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity420 {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

impl CustomEntity420 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
