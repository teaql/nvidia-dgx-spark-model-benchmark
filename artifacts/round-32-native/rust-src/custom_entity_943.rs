// Auto-generated TeaQL Entity: CustomEntity943
// Entity Index: 899
// Source Module: module_62.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity943 {
    pub id: u64,
    pub name: String,
    pub region: String,
    pub zip_code: String,
    pub is_active: bool,
}

impl CustomEntity943 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
