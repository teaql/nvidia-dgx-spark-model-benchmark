// Auto-generated TeaQL Entity: EmailBlast
// Entity Index: 847
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "sender_email")]
pub struct EmailBlast {
    pub id: u64,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub sender_email: String,
    pub send_date: String,
}

impl EmailBlast {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
