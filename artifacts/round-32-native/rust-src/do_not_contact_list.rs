// Auto-generated TeaQL Entity: DoNotContactList
// Entity Index: 517
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email,phone")]
pub struct DoNotContactList {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl DoNotContactList {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
