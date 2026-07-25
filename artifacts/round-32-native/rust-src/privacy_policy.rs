// Auto-generated TeaQL Entity: PrivacyPolicy
// Entity Index: 997
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email")]
pub struct PrivacyPolicy {
    pub id: u64,
    pub name: String,
    pub last_updated: String,
    pub email: String,
}

impl PrivacyPolicy {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
