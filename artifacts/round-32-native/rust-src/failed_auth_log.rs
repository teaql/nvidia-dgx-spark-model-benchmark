// Auto-generated TeaQL Entity: FailedAuthLog
// Entity Index: 46
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email")]
pub struct FailedAuthLog {
    pub id: u64,
    pub name: String,
    pub attempt_time: String,
    pub ip_address: String,
    pub reason: String,
    pub email: String,
}

impl FailedAuthLog {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
