// Auto-generated TeaQL Entity: ProofOfDelivery
// Entity Index: 17
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfDelivery {
    pub id: u64,
    pub name: String,
    pub shipment_reference: String,
    pub timestamp: String,
}

impl ProofOfDelivery {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
