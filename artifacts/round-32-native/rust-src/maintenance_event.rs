// Auto-generated TeaQL Entity: MaintenanceEvent
// Entity Index: 976
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceEvent {
    pub id: u64,
    pub name: String,
    pub service_date: String,
    pub cost: String,
    pub service_provider: String,
    pub description: String,
}

impl MaintenanceEvent {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
