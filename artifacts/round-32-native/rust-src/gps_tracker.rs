// Auto-generated TeaQL Entity: GpsTracker
// Entity Index: 979
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsTracker {
    pub id: u64,
    pub name: String,
    pub device_imei: String,
    pub serial_number: i64,
    pub installed_at: String,
    pub status: String,
}

impl GpsTracker {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
