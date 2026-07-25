// Auto-generated TeaQL Entity: UserRoleAssignment
// Entity Index: 34
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleAssignment {
    pub id: u64,
    pub name: String,
    pub user_account_ref: i64,
    pub role_ref: String,
}

impl UserRoleAssignment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
