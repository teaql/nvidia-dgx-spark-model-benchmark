// Auto-generated TeaQL Entity: WorkedHours
// Entity Index: 186
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkedHours {
    pub id: u64,
    pub name: String,
    pub work_date: String,
    pub regular_hours: f64,
    pub overtime_hours: f64,
    pub total_hours: f64,
}

impl WorkedHours {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
