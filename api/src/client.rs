// Dependencies
use crate::api::Blacklist;

/// Contains each "sub-API" within one struct.
pub struct Client {
    pub blacklist: Blacklist
}
impl Client {
    /// Create an instance of the struct.
    pub fn new(api_key: String, merchant: Option<String>) -> Self {
        Self {
            blacklist: Blacklist { api_key, merchant }
        }
    }
}