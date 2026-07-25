// Auto-generated TeaQL Entity: CustomEntity319
// Entity Index: 215
// Source Module: module_21.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity319 {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub tag: String,
    pub enabled: bool,
}

impl CustomEntity319 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
