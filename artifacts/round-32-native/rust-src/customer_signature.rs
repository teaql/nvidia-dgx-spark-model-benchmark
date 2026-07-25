// Auto-generated TeaQL Entity: CustomerSignature
// Entity Index: 28
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSignature {
    pub id: u64,
    pub name: String,
    pub signature_image_url: String,
    pub timestamp: String,
}

impl CustomerSignature {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
