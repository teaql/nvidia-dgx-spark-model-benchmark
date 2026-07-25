// Auto-generated TeaQL Entity: TaxDocument
// Entity Index: 962
// Source Module: module_7.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxDocument {
    pub id: u64,
    pub name: String,
    pub document_type: String,
    pub issue_date: String,
    pub total_tax: String,
}

impl TaxDocument {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
