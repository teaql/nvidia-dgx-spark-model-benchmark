// Auto-generated TeaQL Entity: CustomEntity793
// Entity Index: 734
// Source Module: module_52.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity793 {
    pub id: u64,
    pub name: String,
    pub region: String,
    pub zip_code: String,
    pub is_active: bool,
}

impl CustomEntity793 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
