// Auto-generated TeaQL Entity: CustomEntity215
// Entity Index: 96
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity215 {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub summary: String,
    pub date_created: String,
    pub active: bool,
}

impl CustomEntity215 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
