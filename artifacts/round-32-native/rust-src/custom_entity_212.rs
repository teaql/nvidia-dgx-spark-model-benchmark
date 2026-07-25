// Auto-generated TeaQL Entity: CustomEntity212
// Entity Index: 93
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity212 {
    pub id: u64,
    pub name: String,
    pub reference: String,
    pub amount: i64,
    pub created_at: String,
    pub status: String,
}

impl CustomEntity212 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
