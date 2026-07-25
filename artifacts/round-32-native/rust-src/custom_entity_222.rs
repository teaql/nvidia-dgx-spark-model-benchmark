// Auto-generated TeaQL Entity: CustomEntity222
// Entity Index: 103
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity222 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub score: String,
    pub valid_from: String,
    pub valid_to: String,
}

impl CustomEntity222 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
