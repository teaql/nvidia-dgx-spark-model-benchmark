// Auto-generated TeaQL Entity: BankTransaction
// Entity Index: 963
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    pub id: u64,
    pub name: String,
    pub transaction_date: String,
    pub amount: i64,
    pub reference: String,
}

impl BankTransaction {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
