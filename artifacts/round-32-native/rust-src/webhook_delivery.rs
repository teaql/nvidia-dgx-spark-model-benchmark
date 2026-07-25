// Auto-generated TeaQL Entity: WebhookDelivery
// Entity Index: 57
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    pub id: u64,
    pub name: String,
    pub delivery_status: String,
    pub response_code: i64,
    pub attempted_at: String,
    pub response_body: String,
}

impl WebhookDelivery {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
