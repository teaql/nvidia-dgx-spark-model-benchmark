// Auto-generated TeaQL Entity: UniformAssignment
// Entity Index: 352
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniformAssignment {
    pub id: u64,
    pub name: String,
    pub item_description: String,
    pub uniform_size: String,
    pub date_issued: String,
}

impl UniformAssignment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
