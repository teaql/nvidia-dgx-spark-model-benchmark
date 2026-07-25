// Auto-generated TeaQL Entity: ExpenseReimbursement
// Entity Index: 347
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "bank_account")]
pub struct ExpenseReimbursement {
    pub id: u64,
    pub name: String,
    pub submit_date: String,
    pub amount: i64,
    pub description: String,
    pub bank_account: i64,
}

impl ExpenseReimbursement {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
