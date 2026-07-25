// Auto-generated TeaQL Entity: Account
// Entity Index: 960
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u64,
    pub name: String,
    pub account_type: i64,
    pub balance: String,
    pub currency: String,
}

impl Account {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
