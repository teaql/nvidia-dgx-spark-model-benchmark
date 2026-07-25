// Auto-generated TeaQL Entity: AssetInspection
// Entity Index: 974
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetInspection {
    pub id: u64,
    pub name: String,
    pub inspection_date: String,
    pub inspector_name: String,
    pub result: String,
    pub comments: String,
}

impl AssetInspection {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
