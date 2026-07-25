// Auto-generated TeaQL Entity: TerminationRecord
// Entity Index: 350
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationRecord {
    pub id: u64,
    pub name: String,
    pub effective_date: String,
    pub reason: String,
    pub remarks: String,
}

impl TerminationRecord {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
