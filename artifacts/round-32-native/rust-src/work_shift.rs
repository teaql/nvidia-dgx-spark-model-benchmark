// Auto-generated TeaQL Entity: WorkShift
// Entity Index: 185
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkShift {
    pub id: u64,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub shift_kind: String,
    pub location_code: String,
}

impl WorkShift {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
