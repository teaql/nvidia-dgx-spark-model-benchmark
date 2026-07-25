// Auto-generated TeaQL Entity: CustomEntity221
// Entity Index: 102
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity221 {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub details: String,
    pub priority: i64,
    pub date_updated: String,
}

impl CustomEntity221 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
