// Auto-generated TeaQL Entity: DashcamFootage
// Entity Index: 980
// Source Module: module_8.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashcamFootage {
    pub id: u64,
    pub name: String,
    pub recorded_at: String,
    pub duration_seconds: i64,
    pub file_path: String,
    pub resolution: String,
}

impl DashcamFootage {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
