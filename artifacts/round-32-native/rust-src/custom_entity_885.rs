// Auto-generated TeaQL Entity: CustomEntity885
// Entity Index: 826
// Source Module: module_59.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity885 {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

impl CustomEntity885 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
