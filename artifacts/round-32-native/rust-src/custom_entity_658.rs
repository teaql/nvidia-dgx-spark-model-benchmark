// Auto-generated TeaQL Entity: CustomEntity658
// Entity Index: 584
// Source Module: module_43.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity658 {
    pub id: u64,
    pub name: String,
    pub region: String,
    pub zip_code: String,
    pub is_active: bool,
}

impl CustomEntity658 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
