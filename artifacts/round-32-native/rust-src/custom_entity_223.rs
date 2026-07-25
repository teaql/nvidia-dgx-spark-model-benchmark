// Auto-generated TeaQL Entity: CustomEntity223
// Entity Index: 104
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity223 {
    pub id: u64,
    pub name: String,
    pub reference_number: i64,
    pub total_amount: i64,
    pub status: String,
    pub is_processed: bool,
}

impl CustomEntity223 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
