// Auto-generated TeaQL Entity: CustomEntity363
// Entity Index: 259
// Source Module: module_24.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity363 {
    pub id: u64,
    pub name: String,
    pub value: String,
    pub unit: String,
    pub notes: String,
}

impl CustomEntity363 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
