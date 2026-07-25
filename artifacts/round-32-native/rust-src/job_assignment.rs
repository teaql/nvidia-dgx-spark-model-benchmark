// Auto-generated TeaQL Entity: JobAssignment
// Entity Index: 184
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobAssignment {
    pub id: u64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub role_title: String,
    pub pay_rate: f64,
}

impl JobAssignment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
