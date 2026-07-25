// Auto-generated TeaQL Entity: ReferralCode
// Entity Index: 512
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralCode {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub discount_amount: i64,
}

impl ReferralCode {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
