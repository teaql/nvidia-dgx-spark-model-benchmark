// Auto-generated TeaQL Entity: CustomEntity219
// Entity Index: 100
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity219 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub count: i64,
    pub amount: i64,
    pub active: bool,
}

impl CustomEntity219 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
