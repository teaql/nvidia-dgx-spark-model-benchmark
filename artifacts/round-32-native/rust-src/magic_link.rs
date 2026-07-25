// Auto-generated TeaQL Entity: MagicLink
// Entity Index: 36
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "token")]
pub struct MagicLink {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub token: String,
    pub expires_at: String,
}

impl MagicLink {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
