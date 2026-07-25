// Auto-generated TeaQL Entity: CleaningService
// Entity Index: 523
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningService {
    pub id: u64,
    pub name: String,
    pub hours: i64,
    pub number_of_cleaners: i64,
}

impl CleaningService {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
