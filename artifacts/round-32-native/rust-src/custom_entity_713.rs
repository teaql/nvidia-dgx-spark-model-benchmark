// Auto-generated TeaQL Entity: CustomEntity713
// Entity Index: 639
// Source Module: module_47.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity713 {
    pub id: u64,
    pub name: String,
    pub comment: String,
    pub author: String,
    pub posted_at: String,
}

impl CustomEntity713 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
