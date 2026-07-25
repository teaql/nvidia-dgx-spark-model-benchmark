// Auto-generated TeaQL Entity: AuditLog
// Entity Index: 42
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub action: String,
    pub level: String,
    pub created_at: String,
}

impl AuditLog {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
