// Auto-generated TeaQL Entity: Merchant
// Entity Index: 4
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "contact_email,contact_phone")]
pub struct Merchant {
    pub id: u64,
    pub name: String,
    pub tax_identifier: String,
    pub status: String,
    pub category: String,
    pub contact_email: String,
    pub contact_phone: String,
}

impl Merchant {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
