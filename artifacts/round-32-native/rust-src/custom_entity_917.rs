// Auto-generated TeaQL Entity: CustomEntity917
// Entity Index: 873
// Source Module: module_61.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity917 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub priority: i64,
    pub due_date: String,
}

impl CustomEntity917 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
