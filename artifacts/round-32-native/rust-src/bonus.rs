// Auto-generated TeaQL Entity: Bonus
// Entity Index: 190
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bonus {
    pub id: u64,
    pub name: String,
    pub bonus_amount: i64,
    pub award_date: String,
    pub reason: String,
    pub status: String,
}

impl Bonus {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
