// Auto-generated TeaQL Entity: FiscalYear
// Entity Index: 969
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalYear {
    pub id: u64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub is_closed: bool,
}

impl FiscalYear {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
