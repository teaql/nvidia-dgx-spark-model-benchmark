// Auto-generated TeaQL Entity: FuelStop
// Entity Index: 26
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelStop {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub cost: String,
}

impl FuelStop {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
