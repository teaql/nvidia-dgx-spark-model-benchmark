// Auto-generated TeaQL Entity: CustomEntity384
// Entity Index: 280
// Source Module: module_25.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity384 {
    pub id: u64,
    pub name: String,
    pub reference_code: String,
    pub amount: i64,
    pub processed: bool,
}

impl CustomEntity384 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
