// Auto-generated TeaQL Entity: LeaveRequest
// Entity Index: 191
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub id: u64,
    pub name: String,
    pub leave_kind: String,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
    pub total_days: f64,
}

impl LeaveRequest {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
