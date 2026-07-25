// Auto-generated TeaQL Entity: CustomEntity533
// Entity Index: 444
// Source Module: module_35.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity533 {
    pub id: u64,
    pub name: String,
    pub comment: String,
    pub author: String,
    pub posted_at: String,
}

impl CustomEntity533 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
