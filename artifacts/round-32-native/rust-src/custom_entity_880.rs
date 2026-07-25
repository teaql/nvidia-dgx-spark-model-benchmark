// Auto-generated TeaQL Entity: CustomEntity880
// Entity Index: 821
// Source Module: module_58.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity880 {
    pub id: u64,
    pub name: String,
    pub group_code: String,
    pub item_count: i64,
    pub is_default: bool,
}

impl CustomEntity880 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
