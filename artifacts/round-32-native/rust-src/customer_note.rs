// Auto-generated TeaQL Entity: CustomerNote
// Entity Index: 518
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerNote {
    pub id: u64,
    pub name: String,
    pub note_text: String,
    pub created_at: String,
}

impl CustomerNote {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
