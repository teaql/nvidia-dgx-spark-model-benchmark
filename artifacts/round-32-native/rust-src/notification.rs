// Auto-generated TeaQL Entity: Notification
// Entity Index: 47
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email")]
pub struct Notification {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub content: String,
    pub channel: String,
    pub status: String,
    pub sent_at: String,
    pub email: String,
}

impl Notification {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
