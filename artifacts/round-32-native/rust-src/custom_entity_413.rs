// Auto-generated TeaQL Entity: CustomEntity413
// Entity Index: 309
// Source Module: module_27.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity413 {
    pub id: u64,
    pub name: String,
    pub comment: String,
    pub author: String,
    pub posted_at: String,
}

impl CustomEntity413 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
