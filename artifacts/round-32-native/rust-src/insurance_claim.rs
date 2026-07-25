// Auto-generated TeaQL Entity: InsuranceClaim
// Entity Index: 989
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceClaim {
    pub id: u64,
    pub name: String,
    pub claim_date: String,
    pub amount_claimed: i64,
    pub status: String,
    pub description: String,
}

impl InsuranceClaim {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
