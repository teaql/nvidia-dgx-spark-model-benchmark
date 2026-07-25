// Auto-generated TeaQL Entity: CreditNote
// Entity Index: 966
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNote {
    pub id: u64,
    pub name: String,
    pub issue_date: String,
    pub amount: i64,
    pub reason: String,
}

impl CreditNote {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
