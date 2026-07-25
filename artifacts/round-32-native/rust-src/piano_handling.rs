// Auto-generated TeaQL Entity: PianoHandling
// Entity Index: 682
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PianoHandling {
    pub id: u64,
    pub name: String,
    pub piano_kind: String,
    pub base_fee: f64,
}

impl PianoHandling {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
