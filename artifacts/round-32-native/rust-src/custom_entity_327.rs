// Auto-generated TeaQL Entity: CustomEntity327
// Entity Index: 223
// Source Module: module_21.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity327 {
    pub id: u64,
    pub name: String,
    pub version_number: i64,
    pub release_date: String,
    pub is_stable: bool,
}

impl CustomEntity327 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
