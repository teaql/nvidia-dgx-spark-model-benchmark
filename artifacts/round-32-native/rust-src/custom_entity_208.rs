// Auto-generated TeaQL Entity: CustomEntity208
// Entity Index: 89
// Source Module: module_13.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity208 {
    pub id: u64,
    pub name: String,
    pub region: String,
    pub zip_code: String,
    pub is_active: bool,
}

impl CustomEntity208 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
