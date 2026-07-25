// Auto-generated TeaQL Entity: PayrollCalculation
// Entity Index: 188
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollCalculation {
    pub id: u64,
    pub name: String,
    pub gross_pay: f64,
    pub net_pay: f64,
    pub total_deductions: f64,
    pub calculation_date: String,
}

impl PayrollCalculation {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
