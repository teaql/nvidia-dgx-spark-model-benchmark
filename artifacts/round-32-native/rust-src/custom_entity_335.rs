// Auto-generated TeaQL Entity: CustomEntity335
// Entity Index: 231
// Source Module: module_22.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity335 {
    pub id: u64,
    pub name: String,
    pub score: i64,
    pub max_score: i64,
    pub evaluated_at: String,
}

impl CustomEntity335 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
