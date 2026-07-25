// Auto-generated TeaQL Entity: CustomEntity755
// Entity Index: 696
// Source Module: module_50.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity755 {
    pub id: u64,
    pub name: String,
    pub score: i64,
    pub max_score: i64,
    pub evaluated_at: String,
}

impl CustomEntity755 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
