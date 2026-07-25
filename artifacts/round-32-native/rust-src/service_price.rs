// Auto-generated TeaQL Entity: ServicePrice
// Entity Index: 677
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePrice {
    pub id: u64,
    pub name: String,
    pub price_list_id: f64,
    pub service_id: String,
    pub amount: i64,
}

impl ServicePrice {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
