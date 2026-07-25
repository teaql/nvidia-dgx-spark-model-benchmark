// Auto-generated TeaQL Entity: PostMoveSurvey
// Entity Index: 30
// Source Module: module_1.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
    #[teaql(audit_mask = "email")]
pub struct PostMoveSurvey {
    pub id: u64,
    pub name: String,
    pub rating: i64,
    pub email: String,
}

impl PostMoveSurvey {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
