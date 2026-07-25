// Auto-generated TeaQL Entity: EmergencyContact
// Entity Index: 351
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "phone,email")]
pub struct EmergencyContact {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub relationship: String,
    pub phone: String,
    pub email: String,
}

impl EmergencyContact {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
