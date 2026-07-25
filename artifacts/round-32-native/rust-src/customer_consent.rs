// Auto-generated TeaQL Entity: CustomerConsent
// Entity Index: 511
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerConsent {
    pub id: u64,
    pub name: String,
    pub consent_type: String,
    pub granted_date: String,
}

impl CustomerConsent {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
