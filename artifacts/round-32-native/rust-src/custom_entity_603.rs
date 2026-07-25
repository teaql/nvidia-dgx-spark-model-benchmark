// Auto-generated TeaQL Entity: CustomEntity603
// Entity Index: 529
// Source Module: module_40.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity603 {
    pub id: u64,
    pub name: String,
    pub value: String,
    pub unit: String,
    pub notes: String,
}

impl CustomEntity603 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
