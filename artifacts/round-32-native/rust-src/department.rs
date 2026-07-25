// Auto-generated TeaQL Entity: Department
// Entity Index: 183
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub id: u64,
    pub name: String,
    pub dept_code: String,
    pub description: String,
    pub budget: f64,
}

impl Department {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
