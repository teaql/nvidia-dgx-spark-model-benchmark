// Auto-generated TeaQL Entity: WarningLetter
// Entity Index: 349
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarningLetter {
    pub id: u64,
    pub name: String,
    pub date_issued: String,
    pub reason: String,
    pub severity_level: String,
}

impl WarningLetter {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
