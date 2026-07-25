// Auto-generated TeaQL Entity: Role
// Entity Index: 32
// Source Module: module_10.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub is_system_role: bool,
}

impl Role {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
