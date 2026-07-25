// Auto-generated TeaQL Entity: ApiClient
// Entity Index: 54
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "secret_key")]
pub struct ApiClient {
    pub id: u64,
    pub name: String,
    pub client_code: String,
    pub status: String,
    pub allowed_ips: String,
    pub secret_key: String,
}

impl ApiClient {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
