// Dependencies
use crate::api::{Blacklist, Whitelist};

/// Contains each "sub-API" within one struct.
pub struct Client {
    pub blacklist: Blacklist,
    pub whitelist: Whitelist,
}
impl Client {
    /// Create an instance of the struct.
    pub fn new(api_key: &str, merchant: Option<&str>) -> Self {
        Self {
            blacklist: Blacklist::new( api_key, merchant ),
            whitelist: Whitelist::new( api_key, merchant ),
        }
    }
}