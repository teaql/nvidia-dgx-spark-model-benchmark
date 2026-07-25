// Auto-generated TeaQL Entity: CustomEntity514
// Entity Index: 425
// Source Module: module_34.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity514 {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub tag: String,
    pub enabled: bool,
}

impl CustomEntity514 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
