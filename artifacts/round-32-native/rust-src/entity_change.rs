// Auto-generated TeaQL Entity: EntityChange
// Entity Index: 43
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityChange {
    pub id: u64,
    pub name: String,
    pub activity_log_ref: String,
    pub field_name: String,
    pub old_value: String,
    pub new_value: String,
}

impl EntityChange {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
