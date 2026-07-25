// Auto-generated TeaQL Entity: Route
// Entity Index: 9
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: u64,
    pub name: String,
    pub route_code: String,
    pub total_distance_km: f64,
    pub estimated_duration_minutes: i64,
    pub status: String,
}

impl Route {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
