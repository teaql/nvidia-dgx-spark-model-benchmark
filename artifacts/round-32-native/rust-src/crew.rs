// Auto-generated TeaQL Entity: Crew
// Entity Index: 14
// Source Module: module_0.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "leader_phone")]
pub struct Crew {
    pub id: u64,
    pub name: String,
    pub crew_code: String,
    pub leader_phone: String,
    pub member_count: i64,
    pub is_active: bool,
}

impl Crew {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
