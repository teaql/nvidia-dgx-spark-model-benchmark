// Auto-generated TeaQL Entity: Payslip
// Entity Index: 189
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payslip {
    pub id: u64,
    pub name: String,
    pub issue_date: String,
    pub gross_amount: i64,
    pub net_amount: i64,
    pub payment_method: String,
}

impl Payslip {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
