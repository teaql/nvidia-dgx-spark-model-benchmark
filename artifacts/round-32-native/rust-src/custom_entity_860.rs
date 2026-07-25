// Auto-generated TeaQL Entity: CustomEntity860
// Entity Index: 801
// Source Module: module_57.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity860 {
    pub id: u64,
    pub name: String,
    pub score: i64,
    pub max_score: i64,
    pub evaluated_at: String,
}

impl CustomEntity860 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
