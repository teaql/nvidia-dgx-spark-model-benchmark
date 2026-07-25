// Auto-generated TeaQL Entity: ComplianceCheck
// Entity Index: 992
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub id: u64,
    pub name: String,
    pub check_date: String,
    pub standard: String,
    pub result: String,
    pub inspector: String,
}

impl ComplianceCheck {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
