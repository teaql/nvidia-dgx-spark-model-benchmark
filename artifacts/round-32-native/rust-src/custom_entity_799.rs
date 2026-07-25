// Auto-generated TeaQL Entity: CustomEntity799
// Entity Index: 740
// Source Module: module_53.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity799 {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub tag: String,
    pub enabled: bool,
}

impl CustomEntity799 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
