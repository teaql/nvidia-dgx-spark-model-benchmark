// Auto-generated TeaQL Entity: Employee
// Entity Index: 182
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email,phone")]
pub struct Employee {
    pub id: u64,
    pub name: String,
    pub hire_date: String,
    pub status: String,
    pub position_title: String,
    pub email: String,
    pub phone: String,
}

impl Employee {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
