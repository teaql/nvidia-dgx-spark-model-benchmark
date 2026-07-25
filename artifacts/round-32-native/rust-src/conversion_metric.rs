// Auto-generated TeaQL Entity: ConversionMetric
// Entity Index: 844
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionMetric {
    pub id: u64,
    pub name: String,
    pub metric_kind: String,
    pub value: f64,
}

impl ConversionMetric {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
