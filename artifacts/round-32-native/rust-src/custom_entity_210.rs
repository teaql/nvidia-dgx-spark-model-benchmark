// Auto-generated TeaQL Entity: CustomEntity210
// Entity Index: 91
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity210 {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub code: String,
    pub description: String,
    pub value: f64,
}

impl CustomEntity210 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
