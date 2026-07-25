// Auto-generated TeaQL Entity: FulfillmentEvent
// Entity Index: 12
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentEvent {
    pub id: u64,
    pub name: String,
    pub event_code: String,
    pub event_timestamp: String,
    pub category: String,
    pub description: String,
    pub status: String,
}

impl FulfillmentEvent {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
