// Auto-generated TeaQL Entity: MoveQuote
// Entity Index: 8
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveQuote {
    pub id: u64,
    pub name: String,
    pub quote_number: i64,
    pub estimated_amount: i64,
    pub discount_amount: i64,
    pub expiration_date: String,
    pub status: String,
}

impl MoveQuote {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
