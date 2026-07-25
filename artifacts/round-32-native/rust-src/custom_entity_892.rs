// Auto-generated TeaQL Entity: CustomEntity892
// Entity Index: 833
// Source Module: module_59.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity892 {
    pub id: u64,
    pub name: String,
    pub capacity: i64,
    pub threshold: String,
    pub is_valid: bool,
}

impl CustomEntity892 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
