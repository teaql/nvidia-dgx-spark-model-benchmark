// Auto-generated TeaQL Entity: CustomEntity562
// Entity Index: 473
// Source Module: module_37.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity562 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub priority: i64,
    pub due_date: String,
}

impl CustomEntity562 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
