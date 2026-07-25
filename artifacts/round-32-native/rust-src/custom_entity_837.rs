// Auto-generated TeaQL Entity: CustomEntity837
// Entity Index: 778
// Source Module: module_55.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity837 {
    pub id: u64,
    pub name: String,
    pub version_number: i64,
    pub release_date: String,
    pub is_stable: bool,
}

impl CustomEntity837 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
