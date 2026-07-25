// Auto-generated TeaQL Entity: WeighStationTicket
// Entity Index: 21
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeighStationTicket {
    pub id: u64,
    pub name: String,
    pub station_location: String,
    pub weight: String,
}

impl WeighStationTicket {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
