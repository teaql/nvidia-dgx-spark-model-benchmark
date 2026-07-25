// Auto-generated TeaQL Entity: CustomEntity428
// Entity Index: 324
// Source Module: module_28.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity428 {
    pub id: u64,
    pub name: String,
    pub comment: String,
    pub author: String,
    pub posted_at: String,
}

impl CustomEntity428 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
