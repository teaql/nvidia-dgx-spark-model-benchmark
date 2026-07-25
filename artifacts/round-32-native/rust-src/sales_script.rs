// Auto-generated TeaQL Entity: SalesScript
// Entity Index: 849
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesScript {
    pub id: u64,
    pub name: String,
    pub product: String,
    pub script_text: String,
}

impl SalesScript {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
