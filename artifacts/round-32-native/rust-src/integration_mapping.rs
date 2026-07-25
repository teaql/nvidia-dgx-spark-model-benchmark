// Auto-generated TeaQL Entity: IntegrationMapping
// Entity Index: 58
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMapping {
    pub id: u64,
    pub name: String,
    pub source_field: String,
    pub target_field: String,
    pub mapping_rule: String,
    pub is_bidirectional: bool,
}

impl IntegrationMapping {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
