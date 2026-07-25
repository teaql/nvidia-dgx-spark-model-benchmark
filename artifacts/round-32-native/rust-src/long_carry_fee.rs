// Auto-generated TeaQL Entity: LongCarryFee
// Entity Index: 684
// Source Module: module_5.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongCarryFee {
    pub id: u64,
    pub name: String,
    pub distance_feet: i64,
    pub fee_amount: i64,
}

impl LongCarryFee {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
