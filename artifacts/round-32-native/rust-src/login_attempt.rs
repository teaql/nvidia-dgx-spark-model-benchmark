// Auto-generated TeaQL Entity: LoginAttempt
// Entity Index: 45
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAttempt {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub ip_address: String,
    pub status: String,
    pub attempted_at: String,
}

impl LoginAttempt {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
