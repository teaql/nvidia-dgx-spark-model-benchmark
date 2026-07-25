// Auto-generated TeaQL Entity: Branch
// Entity Index: 5
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "contact_phone")]
pub struct Branch {
    pub id: u64,
    pub name: String,
    pub branch_code: String,
    pub operating_status: String,
    pub time_zone: String,
    pub contact_phone: String,
}

impl Branch {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
