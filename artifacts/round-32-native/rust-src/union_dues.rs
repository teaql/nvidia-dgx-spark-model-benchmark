// Auto-generated TeaQL Entity: UnionDues
// Entity Index: 195
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionDues {
    pub id: u64,
    pub name: String,
    pub dues_amount: i64,
    pub deduction_date: String,
    pub union_title: String,
    pub status: String,
}

impl UnionDues {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
