// Auto-generated TeaQL Entity: PriceList
// Entity Index: 676
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList {
    pub id: u64,
    pub name: String,
    pub valid_from: String,
    pub valid_to: String,
}

impl PriceList {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
