// Auto-generated TeaQL Entity: CustomEntity217
// Entity Index: 98
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity217 {
    pub id: u64,
    pub name: String,
    pub serial_number: i64,
    pub status: String,
    pub cost: f64,
    pub registered_at: String,
}

impl CustomEntity217 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
