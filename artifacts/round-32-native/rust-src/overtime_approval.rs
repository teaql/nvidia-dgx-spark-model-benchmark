// Auto-generated TeaQL Entity: OvertimeApproval
// Entity Index: 346
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvertimeApproval {
    pub id: u64,
    pub name: String,
    pub approval_date: String,
    pub hours_requested: f64,
    pub reason: String,
    pub status: String,
}

impl OvertimeApproval {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
