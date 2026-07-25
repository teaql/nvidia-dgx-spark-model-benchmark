// Auto-generated TeaQL Entity: Lead
// Entity Index: 690
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "contact_email,contact_phone")]
pub struct Lead {
    pub id: u64,
    pub name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub status: String,
}

impl Lead {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
