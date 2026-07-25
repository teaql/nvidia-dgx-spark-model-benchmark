// Auto-generated TeaQL Entity: CustomEntity964
// Entity Index: 920
// Source Module: module_64.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity964 {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub tag: String,
    pub enabled: bool,
}

impl CustomEntity964 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
