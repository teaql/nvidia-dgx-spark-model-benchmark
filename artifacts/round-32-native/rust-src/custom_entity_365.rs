// Auto-generated TeaQL Entity: CustomEntity365
// Entity Index: 261
// Source Module: module_24.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity365 {
    pub id: u64,
    pub name: String,
    pub score: i64,
    pub max_score: i64,
    pub evaluated_at: String,
}

impl CustomEntity365 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
