// Auto-generated TeaQL Entity: DocumentVersion
// Entity Index: 991
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub id: u64,
    pub name: String,
    pub version_number: i64,
    pub release_date: String,
    pub changes: String,
}

impl DocumentVersion {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
