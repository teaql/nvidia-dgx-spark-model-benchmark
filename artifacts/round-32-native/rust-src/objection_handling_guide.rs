// Auto-generated TeaQL Entity: ObjectionHandlingGuide
// Entity Index: 850
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectionHandlingGuide {
    pub id: u64,
    pub name: String,
    pub objection: String,
    pub response: String,
}

impl ObjectionHandlingGuide {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
