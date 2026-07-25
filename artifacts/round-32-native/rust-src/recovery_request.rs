// Auto-generated TeaQL Entity: RecoveryRequest
// Entity Index: 994
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryRequest {
    pub id: u64,
    pub name: String,
    pub request_date: String,
    pub item_description: String,
    pub status: String,
}

impl RecoveryRequest {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
