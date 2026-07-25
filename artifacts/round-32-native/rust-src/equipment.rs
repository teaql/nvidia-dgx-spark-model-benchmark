// Auto-generated TeaQL Entity: Equipment
// Entity Index: 971
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub id: u64,
    pub name: String,
    pub serial_number: i64,
    pub model_number: i64,
    pub make: String,
    pub purchase_date: String,
    pub status: String,
}

impl Equipment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
