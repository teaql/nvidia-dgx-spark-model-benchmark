// Auto-generated TeaQL Entity: Service
// Entity Index: 521
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: u64,
    pub name: String,
    pub service_name: String,
    pub description: String,
}

impl Service {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
