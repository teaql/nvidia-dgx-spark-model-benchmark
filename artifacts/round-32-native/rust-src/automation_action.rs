// Auto-generated TeaQL Entity: AutomationAction
// Entity Index: 51
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationAction {
    pub id: u64,
    pub name: String,
    pub action_kind: String,
    pub target_system: String,
    pub payload: String,
    pub retry_count: i64,
}

impl AutomationAction {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
