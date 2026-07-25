// Auto-generated TeaQL Entity: DepreciationSchedule
// Entity Index: 985
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepreciationSchedule {
    pub id: u64,
    pub name: String,
    pub fiscal_year: i64,
    pub depreciation_amount: i64,
    pub book_value: String,
    pub method: String,
}

impl DepreciationSchedule {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
