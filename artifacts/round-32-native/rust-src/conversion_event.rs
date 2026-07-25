// Auto-generated TeaQL Entity: ConversionEvent
// Entity Index: 843
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionEvent {
    pub id: u64,
    pub name: String,
    pub event_kind: String,
    pub event_date: String,
}

impl ConversionEvent {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
