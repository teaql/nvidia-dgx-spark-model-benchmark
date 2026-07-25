// Auto-generated TeaQL Entity: Invoice
// Entity Index: 854
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: u64,
    pub name: String,
    pub total_amount: i64,
    pub due_date: String,
    pub issue_date: String,
}

impl Invoice {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
