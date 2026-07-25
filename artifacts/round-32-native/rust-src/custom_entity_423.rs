// Auto-generated TeaQL Entity: CustomEntity423
// Entity Index: 319
// Source Module: module_28.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity423 {
    pub id: u64,
    pub name: String,
    pub value: String,
    pub unit: String,
    pub notes: String,
}

impl CustomEntity423 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
