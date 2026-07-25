// Auto-generated TeaQL Entity: SalesTerritory
// Entity Index: 852
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTerritory {
    pub id: u64,
    pub name: String,
    pub region: String,
    pub manager: String,
}

impl SalesTerritory {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
