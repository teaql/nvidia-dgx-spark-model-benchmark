// Auto-generated TeaQL Entity: NotificationTemplate
// Entity Index: 48
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: u64,
    pub name: String,
    pub template_code: String,
    pub subject: String,
    pub body_content: String,
    pub category: String,
    pub is_active: bool,
}

impl NotificationTemplate {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
