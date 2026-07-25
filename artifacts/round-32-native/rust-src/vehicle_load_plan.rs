// Auto-generated TeaQL Entity: VehicleLoadPlan
// Entity Index: 20
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLoadPlan {
    pub id: u64,
    pub name: String,
    pub vehicle_reference: String,
    pub total_weight: String,
}

impl VehicleLoadPlan {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
