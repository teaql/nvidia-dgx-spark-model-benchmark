// Auto-generated TeaQL Entity: SalesOpportunity
// Entity Index: 841
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOpportunity {
    pub id: u64,
    pub name: String,
    pub amount: i64,
    pub status: String,
    pub close_date: String,
}

impl SalesOpportunity {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
