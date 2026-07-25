// Auto-generated TeaQL Entity: PrivateCustomerProfile
// Entity Index: 355
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateCustomerProfile {
    pub id: u64,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}

impl PrivateCustomerProfile {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
