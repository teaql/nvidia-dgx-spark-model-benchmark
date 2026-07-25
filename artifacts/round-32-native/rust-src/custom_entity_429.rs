// Auto-generated TeaQL Entity: CustomEntity429
// Entity Index: 325
// Source Module: module_28.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity429 {
    pub id: u64,
    pub name: String,
    pub reference_code: String,
    pub amount: i64,
    pub processed: bool,
}

impl CustomEntity429 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
