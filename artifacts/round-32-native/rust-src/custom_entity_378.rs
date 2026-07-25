// Auto-generated TeaQL Entity: CustomEntity378
// Entity Index: 274
// Source Module: module_25.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity378 {
    pub id: u64,
    pub name: String,
    pub value: String,
    pub unit: String,
    pub notes: String,
}

impl CustomEntity378 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
