// Auto-generated TeaQL Entity: ParkingPermit
// Entity Index: 23
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingPermit {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub valid_from: String,
    pub valid_to: String,
}

impl ParkingPermit {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
