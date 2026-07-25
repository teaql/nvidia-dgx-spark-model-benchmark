// Auto-generated TeaQL Entity: ServiceBundle
// Entity Index: 678
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBundle {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub base_price: f64,
}

impl ServiceBundle {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
