// Auto-generated TeaQL Entity: ChangeSet
// Entity Index: 44
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSet {
    pub id: u64,
    pub name: String,
    pub entity_change_ref: String,
    pub version: String,
    pub applied_at: String,
}

impl ChangeSet {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
