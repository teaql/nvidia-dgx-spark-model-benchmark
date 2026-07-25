// Auto-generated TeaQL Entity: InsuranceCard
// Entity Index: 984
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceCard {
    pub id: u64,
    pub name: String,
    pub provider: String,
    pub policy_number: i64,
    pub start_date: String,
    pub expiration_date: String,
    pub coverage_details: String,
}

impl InsuranceCard {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
