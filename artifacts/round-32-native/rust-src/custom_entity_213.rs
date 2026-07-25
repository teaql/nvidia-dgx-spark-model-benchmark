// Auto-generated TeaQL Entity: CustomEntity213
// Entity Index: 94
// Source Module: module_14.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity213 {
    pub id: u64,
    pub name: String,
    pub details: String,
    pub level: i64,
    pub enabled: bool,
    pub date_recorded: String,
}

impl CustomEntity213 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
