// Auto-generated TeaQL Entity: CustomerContact
// Entity Index: 357
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "phone,email")]
pub struct CustomerContact {
    pub id: u64,
    pub name: String,
    pub phone: String,
    pub email: String,
}

impl CustomerContact {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
