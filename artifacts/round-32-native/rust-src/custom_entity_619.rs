// Auto-generated TeaQL Entity: CustomEntity619
// Entity Index: 545
// Source Module: module_41.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity619 {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub tag: String,
    pub enabled: bool,
}

impl CustomEntity619 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
