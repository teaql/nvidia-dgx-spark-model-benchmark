// Auto-generated TeaQL Entity: ResolutionOffer
// Entity Index: 515
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionOffer {
    pub id: u64,
    pub name: String,
    pub offer_details: String,
    pub accepted: bool,
}

impl ResolutionOffer {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
