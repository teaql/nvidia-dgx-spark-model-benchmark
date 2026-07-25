// Auto-generated TeaQL Entity: VipStatus
// Entity Index: 516
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VipStatus {
    pub id: u64,
    pub name: String,
    pub is_vip: bool,
    pub reason: String,
}

impl VipStatus {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
