// Auto-generated TeaQL Entity: TwoFactorAuth
// Entity Index: 39
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "secret_key")]
pub struct TwoFactorAuth {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub secret_key: String,
    pub is_enabled: bool,
}

impl TwoFactorAuth {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
