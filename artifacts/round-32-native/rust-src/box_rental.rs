// Auto-generated TeaQL Entity: BoxRental
// Entity Index: 524
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxRental {
    pub id: u64,
    pub name: String,
    pub quantity: i64,
    pub rental_days: i64,
}

impl BoxRental {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
