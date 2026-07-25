// Auto-generated TeaQL Entity: PetRelocationService
// Entity Index: 687
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetRelocationService {
    pub id: u64,
    pub name: String,
    pub pet_kind: String,
    pub relocation_fee: f64,
}

impl PetRelocationService {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
