// Auto-generated TeaQL Entity: Webhook
// Entity Index: 56
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "secret_key")]
pub struct Webhook {
    pub id: u64,
    pub name: String,
    pub target_url: String,
    pub event_subscription: String,
    pub is_active: bool,
    pub secret_key: String,
}

impl Webhook {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
