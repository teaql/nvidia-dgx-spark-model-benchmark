// Auto-generated TeaQL Entity: CorporateCustomerProfile
// Entity Index: 356
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "tax_id")]
pub struct CorporateCustomerProfile {
    pub id: u64,
    pub name: String,
    pub company_title: String,
    pub tax_id: String,
}

impl CorporateCustomerProfile {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
