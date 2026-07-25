// Auto-generated TeaQL Entity: Permission
// Entity Index: 33
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: u64,
    pub name: String,
    pub resource: String,
    pub operation: String,
}

impl Permission {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
