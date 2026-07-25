// Auto-generated TeaQL Entity: DispatchAssignment
// Entity Index: 15
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchAssignment {
    pub id: u64,
    pub name: String,
    pub assignment_code: String,
    pub assigned_at: String,
    pub status: String,
    pub priority: i64,
}

impl DispatchAssignment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
