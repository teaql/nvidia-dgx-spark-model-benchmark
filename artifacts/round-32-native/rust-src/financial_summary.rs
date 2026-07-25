// Auto-generated TeaQL Entity: FinancialSummary
// Entity Index: 961
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialSummary {
    pub id: u64,
    pub name: String,
    pub period_start: String,
    pub period_end: String,
    pub total_revenue: String,
    pub total_expenses: String,
}

impl FinancialSummary {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
