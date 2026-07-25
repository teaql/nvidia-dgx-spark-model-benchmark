// Auto-generated TeaQL Entity: Franchise
// Entity Index: 6
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "contact_email")]
pub struct Franchise {
    pub id: u64,
    pub name: String,
    pub franchise_code: String,
    pub territory_code: String,
    pub royalty_rate: f64,
    pub status: String,
    pub contact_email: String,
}

impl Franchise {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
