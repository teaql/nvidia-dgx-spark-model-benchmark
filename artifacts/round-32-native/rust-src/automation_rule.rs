// Auto-generated TeaQL Entity: AutomationRule
// Entity Index: 49
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: u64,
    pub name: String,
    pub rule_code: String,
    pub description: String,
    pub category: String,
    pub is_enabled: bool,
    pub created_at: String,
}

impl AutomationRule {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
