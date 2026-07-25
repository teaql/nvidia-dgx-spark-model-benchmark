// Auto-generated TeaQL Entity: CustomEntity216
// Entity Index: 97
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity216 {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub rank: i64,
    pub score: String,
    pub note: String,
}

impl CustomEntity216 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
