// Auto-generated TeaQL Entity: TimeSlot
// Entity Index: 11
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub id: u64,
    pub name: String,
    pub slot_code: String,
    pub start_time: String,
    pub end_time: String,
    pub capacity: i64,
    pub is_available: bool,
}

impl TimeSlot {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
