// Auto-generated TeaQL Entity: PlatformConfig
// Entity Index: 2
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "secret_key")]
pub struct PlatformConfig {
    pub id: u64,
    pub name: String,
    pub config_key: String,
    pub config_value: String,
    pub category: String,
    pub is_enabled: bool,
    pub secret_key: String,
}

impl PlatformConfig {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
