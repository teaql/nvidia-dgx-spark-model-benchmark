// Auto-generated TeaQL Entity: RouteStop
// Entity Index: 10
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStop {
    pub id: u64,
    pub name: String,
    pub stop_sequence: i64,
    pub arrival_window_start: String,
    pub arrival_window_end: String,
    pub status: String,
}

impl RouteStop {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
