// Auto-generated TeaQL Entity: CustomEntity218
// Entity Index: 99
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity218 {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub priority: i64,
    pub remark: String,
    pub status: String,
}

impl CustomEntity218 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
