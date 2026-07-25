// Auto-generated TeaQL Entity: CustomEntity542
// Entity Index: 453
// Source Module: module_36.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity542 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub priority: i64,
    pub due_date: String,
}

impl CustomEntity542 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
