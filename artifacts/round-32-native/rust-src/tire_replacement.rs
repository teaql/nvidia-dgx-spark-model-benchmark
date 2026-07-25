// Auto-generated TeaQL Entity: TireReplacement
// Entity Index: 981
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TireReplacement {
    pub id: u64,
    pub name: String,
    pub replacement_date: String,
    pub mileage: i64,
    pub tire_brand: String,
    pub cost: String,
}

impl TireReplacement {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
