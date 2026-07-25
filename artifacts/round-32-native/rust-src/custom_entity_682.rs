// Auto-generated TeaQL Entity: CustomEntity682
// Entity Index: 608
// Source Module: module_45.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity682 {
    pub id: u64,
    pub name: String,
    pub capacity: i64,
    pub threshold: String,
    pub is_valid: bool,
}

impl CustomEntity682 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
