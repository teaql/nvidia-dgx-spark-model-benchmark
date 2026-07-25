// Auto-generated TeaQL Entity: CustomEntity862
// Entity Index: 803
// Source Module: module_57.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity862 {
    pub id: u64,
    pub name: String,
    pub capacity: i64,
    pub threshold: String,
    pub is_valid: bool,
}

impl CustomEntity862 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
