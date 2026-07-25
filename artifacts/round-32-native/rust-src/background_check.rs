// Auto-generated TeaQL Entity: BackgroundCheck
// Entity Index: 353
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "ssn")]
pub struct BackgroundCheck {
    pub id: u64,
    pub name: String,
    pub check_date: String,
    pub status: String,
    pub ssn: String,
}

impl BackgroundCheck {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
