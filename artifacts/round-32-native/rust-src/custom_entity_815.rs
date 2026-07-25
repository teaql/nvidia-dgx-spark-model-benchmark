// Auto-generated TeaQL Entity: CustomEntity815
// Entity Index: 756
// Source Module: module_54.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntity815 {
    pub id: u64,
    pub name: String,
    pub score: i64,
    pub max_score: i64,
    pub evaluated_at: String,
}

impl CustomEntity815 {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
