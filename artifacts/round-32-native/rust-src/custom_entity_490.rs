// Auto-generated TeaQL Entity: CustomEntity490
// Entity Index: 401
// Source Module: module_32.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity490 {
    pub id: u64,
    pub name: String,
    pub group_code: String,
    pub item_count: i64,
    pub is_default: bool,
}

impl CustomEntity490 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
