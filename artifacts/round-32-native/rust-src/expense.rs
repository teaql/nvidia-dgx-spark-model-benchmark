// Auto-generated TeaQL Entity: Expense
// Entity Index: 957
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: u64,
    pub name: String,
    pub amount: i64,
    pub category: String,
    pub date: String,
}

impl Expense {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
