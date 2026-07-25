// Auto-generated TeaQL Entity: Customer
// Entity Index: 354
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: u64,
    pub name: String,
    pub customer_tier: String,
    pub registration_date: String,
}

impl Customer {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
