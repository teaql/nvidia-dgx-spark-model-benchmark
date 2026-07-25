// Auto-generated TeaQL Entity: Supplier
// Entity Index: 978
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email,phone")]
pub struct Supplier {
    pub id: u64,
    pub name: String,
    pub company_name: String,
    pub contact_person: String,
    pub email: String,
    pub phone: String,
}

impl Supplier {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
