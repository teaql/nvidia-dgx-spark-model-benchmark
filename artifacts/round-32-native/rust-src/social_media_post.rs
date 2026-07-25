// Auto-generated TeaQL Entity: SocialMediaPost
// Entity Index: 846
// Source Module: module_6.xml

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMediaPost {
    pub id: u64,
    pub name: String,
    pub platform: String,
    pub content: String,
    pub post_date: String,
}

impl SocialMediaPost {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
