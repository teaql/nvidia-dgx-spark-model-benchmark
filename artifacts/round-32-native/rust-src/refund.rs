// Auto-generated TeaQL Entity: Refund
// Entity Index: 956
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: u64,
    pub name: String,
    pub amount: i64,
    pub currency: String,
    pub reason: String,
}

impl Refund {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
