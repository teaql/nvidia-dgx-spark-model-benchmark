// Auto-generated TeaQL Entity: CustomEntity973
// Entity Index: 929
// Source Module: module_64.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity973 {
    pub id: u64,
    pub name: String,
    pub region: String,
    pub zip_code: String,
    pub is_active: bool,
}

impl CustomEntity973 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
