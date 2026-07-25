// Auto-generated TeaQL Entity: DamageReport
// Entity Index: 16
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageReport {
    pub id: u64,
    pub name: String,
    pub item_description: String,
    pub severity: String,
    pub date_reported: String,
}

impl DamageReport {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
