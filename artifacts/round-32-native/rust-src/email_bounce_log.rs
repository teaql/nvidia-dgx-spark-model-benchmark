// Auto-generated TeaQL Entity: EmailBounceLog
// Entity Index: 53
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email")]
pub struct EmailBounceLog {
    pub id: u64,
    pub name: String,
    pub bounce_kind: String,
    pub diagnostic_code: String,
    pub bounced_at: String,
    pub email: String,
}

impl EmailBounceLog {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
