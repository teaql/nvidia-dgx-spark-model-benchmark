// Auto-generated TeaQL Entity: ChargebackRecord
// Entity Index: 965
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargebackRecord {
    pub id: u64,
    pub name: String,
    pub dispute_date: String,
    pub amount: i64,
    pub reason_code: String,
}

impl ChargebackRecord {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
