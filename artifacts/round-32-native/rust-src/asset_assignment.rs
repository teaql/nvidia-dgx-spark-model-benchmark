// Auto-generated TeaQL Entity: AssetAssignment
// Entity Index: 973
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAssignment {
    pub id: u64,
    pub name: String,
    pub assigned_to: String,
    pub assignment_date: String,
    pub return_date: String,
    pub notes: String,
}

impl AssetAssignment {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
