// Auto-generated TeaQL Entity: CustomEntity264
// Entity Index: 145
// Source Module: module_17.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity264 {
    pub id: u64,
    pub name: String,
    pub reference_code: String,
    pub amount: i64,
    pub processed: bool,
}

impl CustomEntity264 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
