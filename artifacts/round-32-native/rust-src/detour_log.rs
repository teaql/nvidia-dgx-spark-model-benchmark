// Auto-generated TeaQL Entity: DetourLog
// Entity Index: 25
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetourLog {
    pub id: u64,
    pub name: String,
    pub reason: String,
    pub time_lost: i64,
}

impl DetourLog {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
