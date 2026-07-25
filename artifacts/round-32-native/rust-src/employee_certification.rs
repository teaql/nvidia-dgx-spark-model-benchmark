// Auto-generated TeaQL Entity: EmployeeCertification
// Entity Index: 192
// Source Module: module_2.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeCertification {
    pub id: u64,
    pub name: String,
    pub certification_title: String,
    pub issuing_authority: String,
    pub issue_date: String,
    pub expiry_date: String,
}

impl EmployeeCertification {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
