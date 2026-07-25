// Auto-generated TeaQL Entity: CustomEntity881
// Entity Index: 822
// Source Module: module_58.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity881 {
    pub id: u64,
    pub name: String,
    pub level: String,
    pub weight: String,
    pub updated_at: String,
}

impl CustomEntity881 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
