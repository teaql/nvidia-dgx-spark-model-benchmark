// Auto-generated TeaQL Entity: RegistrationRenewal
// Entity Index: 983
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRenewal {
    pub id: u64,
    pub name: String,
    pub renewal_date: String,
    pub expiration_date: String,
    pub fee: f64,
    pub state_code: String,
}

impl RegistrationRenewal {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
