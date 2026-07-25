// Auto-generated TeaQL Entity: CustomEntity776
// Entity Index: 717
// Source Module: module_51.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity776 {
    pub id: u64,
    pub name: String,
    pub level: String,
    pub weight: String,
    pub updated_at: String,
}

impl CustomEntity776 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
