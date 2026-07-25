// Auto-generated TeaQL Entity: CustomEntity430
// Entity Index: 326
// Source Module: module_28.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity430 {
    pub id: u64,
    pub name: String,
    pub group_code: String,
    pub item_count: i64,
    pub is_default: bool,
}

impl CustomEntity430 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
