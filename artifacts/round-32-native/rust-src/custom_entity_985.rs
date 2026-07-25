// Auto-generated TeaQL Entity: CustomEntity985
// Entity Index: 941
// Source Module: module_65.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity985 {
    pub id: u64,
    pub name: String,
    pub group_code: String,
    pub item_count: i64,
    pub is_default: bool,
}

impl CustomEntity985 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
