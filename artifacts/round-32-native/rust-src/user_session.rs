// Auto-generated TeaQL Entity: UserSession
// Entity Index: 37
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "session_token")]
pub struct UserSession {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub session_token: String,
    pub expires_at: String,
}

impl UserSession {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
