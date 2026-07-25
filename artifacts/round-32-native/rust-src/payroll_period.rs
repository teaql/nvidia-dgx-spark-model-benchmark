// Auto-generated TeaQL Entity: PayrollPeriod
// Entity Index: 187
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollPeriod {
    pub id: u64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub pay_date: String,
    pub period_status: String,
}

impl PayrollPeriod {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
