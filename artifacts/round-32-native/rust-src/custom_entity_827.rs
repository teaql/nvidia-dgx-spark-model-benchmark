// Auto-generated TeaQL Entity: CustomEntity827
// Entity Index: 768
// Source Module: module_55.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity827 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub priority: i64,
    pub due_date: String,
}

impl CustomEntity827 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
