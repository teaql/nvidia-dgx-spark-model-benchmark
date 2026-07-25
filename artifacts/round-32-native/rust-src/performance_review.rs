// Auto-generated TeaQL Entity: PerformanceReview
// Entity Index: 348
// Source Module: module_3.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReview {
    pub id: u64,
    pub name: String,
    pub review_date: String,
    pub score: i64,
    pub comments: String,
}

impl PerformanceReview {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
