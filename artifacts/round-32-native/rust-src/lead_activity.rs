// Auto-generated TeaQL Entity: LeadActivity
// Entity Index: 842
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadActivity {
    pub id: u64,
    pub name: String,
    pub activity_kind: String,
    pub activity_date: String,
    pub notes: String,
}

impl LeadActivity {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
