// Auto-generated TeaQL Entity: CommunicationLog
// Entity Index: 519
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationLog {
    pub id: u64,
    pub name: String,
    pub channel: String,
    pub message_content: String,
    pub sent_at: String,
}

impl CommunicationLog {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
