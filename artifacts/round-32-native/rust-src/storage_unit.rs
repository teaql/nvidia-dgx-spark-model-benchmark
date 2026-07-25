// Auto-generated TeaQL Entity: StorageUnit
// Entity Index: 679
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUnit {
    pub id: u64,
    pub name: String,
    pub size_sqft: i64,
    pub monthly_rate: f64,
    pub location_id: String,
}

impl StorageUnit {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
