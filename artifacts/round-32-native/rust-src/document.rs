// Auto-generated TeaQL Entity: Document
// Entity Index: 990
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: u64,
    pub name: String,
    pub file_path: String,
    pub author: String,
    pub creation_date: String,
}

impl Document {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
