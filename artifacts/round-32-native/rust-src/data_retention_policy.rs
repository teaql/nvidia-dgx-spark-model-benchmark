// Auto-generated TeaQL Entity: DataRetentionPolicy
// Entity Index: 993
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub id: u64,
    pub name: String,
    pub retention_period_days: i64,
    pub data_category: String,
}

impl DataRetentionPolicy {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
