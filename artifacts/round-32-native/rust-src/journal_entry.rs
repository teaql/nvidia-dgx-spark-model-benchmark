// Auto-generated TeaQL Entity: JournalEntry
// Entity Index: 959
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: u64,
    pub name: String,
    pub posting_date: String,
    pub total_debit: String,
    pub total_credit: String,
}

impl JournalEntry {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
