// Auto-generated TeaQL Entity: OperationsManagerOverride
// Entity Index: 181
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationsManagerOverride {
    pub id: u64,
    pub name: String,
    pub override_reason: String,
    pub approval_date: String,
    pub status: String,
}

impl OperationsManagerOverride {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
