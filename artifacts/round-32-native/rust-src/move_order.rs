// Auto-generated TeaQL Entity: MoveOrder
// Entity Index: 7
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveOrder {
    pub id: u64,
    pub name: String,
    pub order_number: i64,
    pub status: String,
    pub category: String,
    pub priority: i64,
    pub scheduled_date: String,
    pub total_amount: i64,
}

impl MoveOrder {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
