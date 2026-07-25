// Auto-generated TeaQL Entity: DebitNote
// Entity Index: 967
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitNote {
    pub id: u64,
    pub name: String,
    pub issue_date: String,
    pub amount: i64,
    pub reason: String,
}

impl DebitNote {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
