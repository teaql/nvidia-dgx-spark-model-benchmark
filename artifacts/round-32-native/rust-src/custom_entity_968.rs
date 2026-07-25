// Auto-generated TeaQL Entity: CustomEntity968
// Entity Index: 924
// Source Module: module_64.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity968 {
    pub id: u64,
    pub name: String,
    pub comment: String,
    pub author: String,
    pub posted_at: String,
}

impl CustomEntity968 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
