// Auto-generated TeaQL Entity: TermsOfService
// Entity Index: 996
// Source Module: module_9.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsOfService {
    pub id: u64,
    pub name: String,
    pub version_string: String,
    pub effective_date: String,
    pub content_url: String,
}

impl TermsOfService {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
