// Auto-generated TeaQL Entity: ApiEndpoint
// Entity Index: 55
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub id: u64,
    pub name: String,
    pub path_pattern: String,
    pub http_method: String,
    pub version_tag: String,
    pub is_deprecated: bool,
}

impl ApiEndpoint {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
