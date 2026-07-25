// Auto-generated TeaQL Entity: HoistingService
// Entity Index: 685
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoistingService {
    pub id: u64,
    pub name: String,
    pub item_description: String,
    pub hoisting_fee: f64,
}

impl HoistingService {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
