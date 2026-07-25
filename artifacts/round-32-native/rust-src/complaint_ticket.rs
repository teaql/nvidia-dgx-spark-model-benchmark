// Auto-generated TeaQL Entity: ComplaintTicket
// Entity Index: 514
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "customer_email")]
pub struct ComplaintTicket {
    pub id: u64,
    pub name: String,
    pub issue_description: String,
    pub status: String,
    pub customer_email: String,
}

impl ComplaintTicket {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
