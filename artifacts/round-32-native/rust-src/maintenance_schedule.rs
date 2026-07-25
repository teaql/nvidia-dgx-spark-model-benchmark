// Auto-generated TeaQL Entity: MaintenanceSchedule
// Entity Index: 975
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub id: u64,
    pub name: String,
    pub frequency_days: i64,
    pub last_service_date: String,
    pub next_due_date: String,
    pub service_details: String,
}

impl MaintenanceSchedule {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
