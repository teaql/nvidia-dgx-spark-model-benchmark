// Auto-generated TeaQL Entity: TollReceipt
// Entity Index: 22
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollReceipt {
    pub id: u64,
    pub name: String,
    pub plaza_name: String,
    pub cost: String,
}

impl TollReceipt {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
