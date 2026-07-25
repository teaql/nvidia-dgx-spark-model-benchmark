// Auto-generated TeaQL Entity: DirectDepositInfo
// Entity Index: 194
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "account_number,routing_number")]
pub struct DirectDepositInfo {
    pub id: u64,
    pub name: String,
    pub bank_institution: String,
    pub account_number: i64,
    pub routing_number: i64,
    pub account_kind: i64,
}

impl DirectDepositInfo {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
