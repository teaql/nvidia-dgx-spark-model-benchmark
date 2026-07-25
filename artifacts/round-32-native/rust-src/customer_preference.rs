// Auto-generated TeaQL Entity: CustomerPreference
// Entity Index: 360
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPreference {
    pub id: u64,
    pub name: String,
    pub preferred_contact_method: String,
    pub language: String,
}

impl CustomerPreference {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
