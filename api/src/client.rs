// Dependencies
use crate::api::{Blacklist, Whitelist, Category, Coupon, Feedback, Customer, Subscription, Group, Order};

/// Contains each "sub-API" within one struct.
pub struct Client {
    pub blacklist: Blacklist,
    pub whitelist: Whitelist,
    pub category: Category,
    pub coupon: Coupon,
    pub feedback: Feedback,
    pub order: Order,
    pub group: Group,
    pub customer: Customer,
    pub subscription: Subscription
}
impl Client {
    /// Create an instance of the struct.
    pub fn new(api_key: &str, merchant: Option<&str>) -> Self {
        Self {
            blacklist: Blacklist::client( api_key, merchant ),
            whitelist: Whitelist::client( api_key, merchant ),
            category: Category::client( api_key, merchant ),
            coupon: Coupon::client( api_key, merchant ),
            feedback: Feedback::client( api_key, merchant ),
            order: Order::client( api_key, merchant ),
            group: Group::client( api_key, merchant ),
            customer: Customer::client( api_key, merchant ),
            subscription: Subscription::client( api_key, merchant ),
        }
    }
}