// Auto-generated TeaQL Entity: Campaign
// Entity Index: 688
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: u64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub budget: f64,
}

impl Campaign {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
