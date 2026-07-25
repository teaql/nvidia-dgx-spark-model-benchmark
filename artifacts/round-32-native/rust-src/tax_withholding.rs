// Auto-generated TeaQL Entity: TaxWithholding
// Entity Index: 193
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxWithholding {
    pub id: u64,
    pub name: String,
    pub tax_year: i64,
    pub federal_withholding: f64,
    pub state_withholding: f64,
    pub filing_status: String,
}

impl TaxWithholding {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
