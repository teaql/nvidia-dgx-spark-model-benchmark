// Auto-generated TeaQL Entity: WalkthroughChecklist
// Entity Index: 29
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkthroughChecklist {
    pub id: u64,
    pub name: String,
    pub passed: bool,
    pub comments: String,
}

impl WalkthroughChecklist {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
