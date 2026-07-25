// Auto-generated TeaQL Entity: TenantRegistry
// Entity Index: 3
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "contact_email,contact_phone")]
pub struct TenantRegistry {
    pub id: u64,
    pub name: String,
    pub tenant_code: String,
    pub domain_prefix: String,
    pub status: String,
    pub max_branches: i64,
    pub contact_email: String,
    pub contact_phone: String,
}

impl TenantRegistry {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
