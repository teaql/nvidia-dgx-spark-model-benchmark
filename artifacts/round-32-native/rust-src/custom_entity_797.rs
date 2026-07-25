// Auto-generated TeaQL Entity: CustomEntity797
// Entity Index: 738
// Source Module: module_53.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity797 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub priority: i64,
    pub due_date: String,
}

impl CustomEntity797 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
