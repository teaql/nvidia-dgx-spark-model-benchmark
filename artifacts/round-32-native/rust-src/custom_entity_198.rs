// Auto-generated TeaQL Entity: CustomEntity198
// Entity Index: 79
// Source Module: module_13.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity198 {
    pub id: u64,
    pub name: String,
    pub value: String,
    pub unit: String,
    pub notes: String,
}

impl CustomEntity198 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
