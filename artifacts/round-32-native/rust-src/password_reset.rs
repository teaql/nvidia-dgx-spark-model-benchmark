// Auto-generated TeaQL Entity: PasswordReset
// Entity Index: 38
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "token")]
pub struct PasswordReset {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub token: String,
    pub expires_at: String,
}

impl PasswordReset {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
