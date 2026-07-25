// Auto-generated TeaQL Entity: CustomEntity895
// Entity Index: 836
// Source Module: module_59.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity895 {
    pub id: u64,
    pub name: String,
    pub group_code: String,
    pub item_count: i64,
    pub is_default: bool,
}

impl CustomEntity895 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
