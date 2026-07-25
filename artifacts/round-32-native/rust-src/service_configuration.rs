// Auto-generated TeaQL Entity: ServiceConfiguration
// Entity Index: 525
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfiguration {
    pub id: u64,
    pub name: String,
    pub setting_key: String,
    pub setting_value: String,
}

impl ServiceConfiguration {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
