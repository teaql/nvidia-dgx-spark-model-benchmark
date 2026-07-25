// Auto-generated TeaQL Entity: VatRate
// Entity Index: 958
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRate {
    pub id: u64,
    pub name: String,
    pub rate_percentage: f64,
    pub country_code: i64,
}

impl VatRate {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
