// Auto-generated TeaQL Entity: Address
// Entity Index: 13
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: u64,
    pub name: String,
    pub street_address: String,
    pub unit: String,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: i64,
    pub latitude: f64,
    pub longitude: f64,
}

impl Address {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
