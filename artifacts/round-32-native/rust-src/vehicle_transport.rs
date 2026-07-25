// Auto-generated TeaQL Entity: VehicleTransport
// Entity Index: 686
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleTransport {
    pub id: u64,
    pub name: String,
    pub vehicle_make: String,
    pub vehicle_model: String,
    pub transport_fee: f64,
}

impl VehicleTransport {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
