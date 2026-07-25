// Auto-generated TeaQL Entity: RolePermission
// Entity Index: 35
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
    pub id: u64,
    pub name: String,
    pub role_ref: String,
    pub permission_ref: String,
}

impl RolePermission {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
