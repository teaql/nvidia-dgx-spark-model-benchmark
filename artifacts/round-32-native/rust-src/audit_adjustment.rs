// Auto-generated TeaQL Entity: AuditAdjustment
// Entity Index: 968
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditAdjustment {
    pub id: u64,
    pub name: String,
    pub adjustment_date: String,
    pub amount: i64,
    pub explanation: String,
}

impl AuditAdjustment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
