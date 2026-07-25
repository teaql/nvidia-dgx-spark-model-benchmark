// Auto-generated TeaQL Entity: CustomEntity747
// Entity Index: 673
// Source Module: module_49.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity747 {
    pub id: u64,
    pub name: String,
    pub version_number: i64,
    pub release_date: String,
    pub is_stable: bool,
}

impl CustomEntity747 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
