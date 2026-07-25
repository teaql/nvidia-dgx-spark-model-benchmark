// Auto-generated TeaQL Entity: BillingProfile
// Entity Index: 358
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "credit_card,billing_address")]
pub struct BillingProfile {
    pub id: u64,
    pub name: String,
    pub credit_card: String,
    pub billing_address: String,
}

impl BillingProfile {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
