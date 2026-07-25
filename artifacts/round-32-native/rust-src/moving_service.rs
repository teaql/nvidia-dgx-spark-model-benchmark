// Auto-generated TeaQL Entity: MovingService
// Entity Index: 522
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingService {
    pub id: u64,
    pub name: String,
    pub distance_km: f64,
    pub vehicle_size: String,
}

impl MovingService {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
