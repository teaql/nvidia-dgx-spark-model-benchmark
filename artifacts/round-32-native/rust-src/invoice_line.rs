// Auto-generated TeaQL Entity: InvoiceLine
// Entity Index: 855
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub quantity: i64,
    pub unit_price: f64,
}

impl InvoiceLine {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
