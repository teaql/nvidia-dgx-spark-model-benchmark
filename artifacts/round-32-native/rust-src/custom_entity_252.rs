// Auto-generated TeaQL Entity: CustomEntity252
// Entity Index: 133
// Source Module: module_16.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity252 {
    pub id: u64,
    pub name: String,
    pub version_number: i64,
    pub release_date: String,
    pub is_stable: bool,
}

impl CustomEntity252 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
