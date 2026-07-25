// Auto-generated TeaQL Entity: CustomEntity267
// Entity Index: 148
// Source Module: module_17.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity267 {
    pub id: u64,
    pub name: String,
    pub version_number: i64,
    pub release_date: String,
    pub is_stable: bool,
}

impl CustomEntity267 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
