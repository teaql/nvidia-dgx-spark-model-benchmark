// Auto-generated TeaQL Entity: LoyaltyTier
// Entity Index: 513
// Source Module: module_4.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyTier {
    pub id: u64,
    pub name: String,
    pub tier_level: String,
    pub points_required: i64,
}

impl LoyaltyTier {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
