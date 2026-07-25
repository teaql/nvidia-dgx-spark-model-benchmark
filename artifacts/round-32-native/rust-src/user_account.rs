// Auto-generated TeaQL Entity: UserAccount
// Entity Index: 31
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email,password_hash")]
pub struct UserAccount {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub status: String,
}

impl UserAccount {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
