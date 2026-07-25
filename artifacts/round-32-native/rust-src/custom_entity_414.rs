// Auto-generated TeaQL Entity: CustomEntity414
// Entity Index: 310
// Source Module: module_27.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity414 {
    pub id: u64,
    pub name: String,
    pub reference_code: String,
    pub amount: i64,
    pub processed: bool,
}

impl CustomEntity414 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
