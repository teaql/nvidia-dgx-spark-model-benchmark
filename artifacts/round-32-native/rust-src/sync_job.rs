// Auto-generated TeaQL Entity: SyncJob
// Entity Index: 59
// Source Module: module_11.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncJob {
    pub id: u64,
    pub name: String,
    pub job_code: String,
    pub job_kind: String,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
}

impl SyncJob {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
