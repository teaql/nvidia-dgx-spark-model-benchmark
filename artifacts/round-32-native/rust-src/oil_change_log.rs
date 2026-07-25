// Auto-generated TeaQL Entity: OilChangeLog
// Entity Index: 982
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OilChangeLog {
    pub id: u64,
    pub name: String,
    pub service_date: String,
    pub mileage: i64,
    pub oil_brand: String,
    pub cost: String,
}

impl OilChangeLog {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
