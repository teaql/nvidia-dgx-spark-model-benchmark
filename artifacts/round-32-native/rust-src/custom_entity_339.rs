// Auto-generated TeaQL Entity: CustomEntity339
// Entity Index: 235
// Source Module: module_22.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity339 {
    pub id: u64,
    pub name: String,
    pub reference_code: String,
    pub amount: i64,
    pub processed: bool,
}

impl CustomEntity339 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
