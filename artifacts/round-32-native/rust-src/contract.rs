// Auto-generated TeaQL Entity: Contract
// Entity Index: 987
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: u64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub value: String,
    pub status: String,
}

impl Contract {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
