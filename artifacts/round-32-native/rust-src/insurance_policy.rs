// Auto-generated TeaQL Entity: InsurancePolicy
// Entity Index: 988
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePolicy {
    pub id: u64,
    pub name: String,
    pub provider: String,
    pub coverage_amount: i64,
    pub start_date: String,
    pub end_date: String,
}

impl InsurancePolicy {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
