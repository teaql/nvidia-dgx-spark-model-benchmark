// Auto-generated TeaQL Entity: SmsDeliveryReceipt
// Entity Index: 52
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "phone")]
pub struct SmsDeliveryReceipt {
    pub id: u64,
    pub name: String,
    pub message_sid: String,
    pub status: String,
    pub error_code: String,
    pub delivered_at: String,
    pub phone: String,
}

impl SmsDeliveryReceipt {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
