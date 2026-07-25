// Auto-generated TeaQL Entity: GdprRequest
// Entity Index: 999
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email")]
pub struct GdprRequest {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub request_date: String,
    pub action_requested: String,
    pub status: String,
}

impl GdprRequest {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
