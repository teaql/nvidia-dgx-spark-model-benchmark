// Auto-generated TeaQL Entity: ScrapRecord
// Entity Index: 986
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapRecord {
    pub id: u64,
    pub name: String,
    pub item_code: String,
    pub scrap_date: String,
    pub reason: String,
    pub quantity: i64,
}

impl ScrapRecord {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
