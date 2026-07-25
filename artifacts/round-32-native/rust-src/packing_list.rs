// Auto-generated TeaQL Entity: PackingList
// Entity Index: 18
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackingList {
    pub id: u64,
    pub name: String,
    pub box_count: i64,
    pub weight: String,
}

impl PackingList {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
