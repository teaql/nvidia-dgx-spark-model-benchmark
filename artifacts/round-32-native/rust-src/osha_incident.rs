// Auto-generated TeaQL Entity: OshaIncident
// Entity Index: 1000
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OshaIncident {
    pub id: u64,
    pub name: String,
    pub incident_date: String,
    pub location: String,
    pub description: String,
    pub severity: String,
}

impl OshaIncident {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
