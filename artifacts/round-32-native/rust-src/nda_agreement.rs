// Auto-generated TeaQL Entity: NdaAgreement
// Entity Index: 995
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email")]
pub struct NdaAgreement {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub signed_date: String,
    pub valid_until: String,
}

impl NdaAgreement {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
