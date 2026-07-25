// Auto-generated TeaQL Entity: StairFee
// Entity Index: 683
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StairFee {
    pub id: u64,
    pub name: String,
    pub flight_count: i64,
    pub fee_per_flight: f64,
}

impl StairFee {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
