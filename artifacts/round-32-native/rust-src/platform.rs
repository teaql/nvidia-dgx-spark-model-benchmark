// Auto-generated TeaQL Entity: Platform
// Entity Index: 1
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "support_email")]
pub struct Platform {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub status: String,
    pub environment: String,
    pub version: String,
    pub support_email: String,
}

impl Platform {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
