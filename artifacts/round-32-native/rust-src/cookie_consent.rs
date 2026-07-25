// Auto-generated TeaQL Entity: CookieConsent
// Entity Index: 998
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieConsent {
    pub id: u64,
    pub name: String,
    pub user_ip: String,
    pub consent_date: String,
    pub preferences: String,
}

impl CookieConsent {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
