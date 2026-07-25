// Auto-generated TeaQL Entity: MerchantFee
// Entity Index: 964
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantFee {
    pub id: u64,
    pub name: String,
    pub fee_amount: i64,
    pub transaction_reference: String,
}

impl MerchantFee {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
