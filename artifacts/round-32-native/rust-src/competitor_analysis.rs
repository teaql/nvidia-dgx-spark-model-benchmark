// Auto-generated TeaQL Entity: CompetitorAnalysis
// Entity Index: 851
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorAnalysis {
    pub id: u64,
    pub name: String,
    pub competitor: String,
    pub strengths: String,
    pub weaknesses: String,
}

impl CompetitorAnalysis {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
