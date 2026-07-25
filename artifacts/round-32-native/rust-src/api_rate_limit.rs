// Auto-generated TeaQL Entity: ApiRateLimit
// Entity Index: 60
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRateLimit {
    pub id: u64,
    pub name: String,
    pub limit_key: String,
    pub max_requests: i64,
    pub window_seconds: i64,
    pub current_count: i64,
}

impl ApiRateLimit {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
