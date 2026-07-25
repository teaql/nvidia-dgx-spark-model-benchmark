// Auto-generated TeaQL Entity: ActivityLog
// Entity Index: 41
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub action: String,
    pub created_at: String,
}

impl ActivityLog {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
