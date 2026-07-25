// Auto-generated TeaQL Entity: CustomEntity922
// Entity Index: 878
// Source Module: module_61.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity922 {
    pub id: u64,
    pub name: String,
    pub capacity: i64,
    pub threshold: String,
    pub is_valid: bool,
}

impl CustomEntity922 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
