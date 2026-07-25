// Auto-generated TeaQL Entity: WeatherDelay
// Entity Index: 27
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherDelay {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub duration_hours: String,
}

impl WeatherDelay {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
