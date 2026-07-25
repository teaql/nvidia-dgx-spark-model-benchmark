// Auto-generated TeaQL Entity: TrafficViolation
// Entity Index: 24
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficViolation {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub fine: String,
}

impl TrafficViolation {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
