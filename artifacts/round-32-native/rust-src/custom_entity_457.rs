// Auto-generated TeaQL Entity: CustomEntity457
// Entity Index: 368
// Source Module: module_30.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity457 {
    pub id: u64,
    pub name: String,
    pub capacity: i64,
    pub threshold: String,
    pub is_valid: bool,
}

impl CustomEntity457 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
