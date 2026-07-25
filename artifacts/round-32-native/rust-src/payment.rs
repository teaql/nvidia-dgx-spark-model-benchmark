// Auto-generated TeaQL Entity: Payment
// Entity Index: 853
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "payment_token")]
pub struct Payment {
    pub id: u64,
    pub name: String,
    pub amount: i64,
    pub payment_date: String,
    pub payment_token: String,
}

impl Payment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
