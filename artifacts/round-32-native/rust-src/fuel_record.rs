// Auto-generated TeaQL Entity: FuelRecord
// Entity Index: 977
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelRecord {
    pub id: u64,
    pub name: String,
    pub recorded_at: String,
    pub gallons: String,
    pub unit_price: f64,
    pub total_cost: String,
    pub odometer: i64,
}

impl FuelRecord {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
