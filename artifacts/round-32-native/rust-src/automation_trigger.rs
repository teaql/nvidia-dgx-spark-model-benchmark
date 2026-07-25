// Auto-generated TeaQL Entity: AutomationTrigger
// Entity Index: 50
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationTrigger {
    pub id: u64,
    pub name: String,
    pub trigger_event: String,
    pub condition_expression: String,
    pub is_active: bool,
    pub execution_order: i64,
}

impl AutomationTrigger {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
