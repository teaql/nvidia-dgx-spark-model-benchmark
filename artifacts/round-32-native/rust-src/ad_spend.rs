// Auto-generated TeaQL Entity: AdSpend
// Entity Index: 845
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdSpend {
    pub id: u64,
    pub name: String,
    pub platform: String,
    pub spend_amount: i64,
    pub start_date: String,
    pub end_date: String,
}

impl AdSpend {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
