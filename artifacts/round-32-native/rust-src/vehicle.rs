// Auto-generated TeaQL Entity: Vehicle
// Entity Index: 970
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: u64,
    pub name: String,
    pub make: String,
    pub vehicle_model: String,
    pub year: i64,
    pub license_plate: String,
}

impl Vehicle {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
