// Auto-generated TeaQL Entity: SmsCampaign
// Entity Index: 848
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "target_phone")]
pub struct SmsCampaign {
    pub id: u64,
    pub name: String,
    pub campaign_topic: String,
    pub message_content: String,
    pub target_phone: String,
}

impl SmsCampaign {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
