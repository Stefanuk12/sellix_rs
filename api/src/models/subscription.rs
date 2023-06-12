// Dependencies
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use std::time::SystemTime;

/// All of the possible intervals subscriptions can be billed within
/// <https://developers.sellix.io/#subscription-features>.
pub enum RecurringBillingIntervals {
    Daily,
    Weekly,
    Monthly,
    Yearly
}

/// All of the possible statuses a subscription can be in
/// <https://developers.sellix.io/#subscription-configure-webhooks>.
pub enum SubscriptionStatus {
    Pending,
    Cancelled,
    Trialing,
    Active
}

/// Represents the possible customer details from an API object.
pub struct CustomerData {
    /// Customer name
    pub name: String,
    /// Customer surname
    pub surname: String,
    /// Customer phone
    pub phone: Option<String>,
    /// Customer phone country code
    pub phone_country_code: Option<String>,
    /// Customer country code
    pub country_code: Option<String>,
    /// Customer street address
    pub address: Option<String>,
    /// Customer street address additional info
    pub additional_address_info: Option<String>,
    /// Customer city
    pub city: Option<String>,
    /// Customer postal code
    pub postal_code: Option<String>,
    /// Customer state
    pub state: Option<String>,
    /// Customer email
    pub email: String,
}

/// Represents the raw API response for a subscription object
/// <https://developers.sellix.io/#subscription-configure-webhooks>.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRaw {
    /// ID of the subscription (recurring bill)
    pub id: String,
    /// ID of the shop whose resource is assigned to
    pub shop_id: u64,
    /// ID of the shop whose resource is assigned to
    pub product_id: String,
    /// Subscription's status, see more info about statuses below
    pub status: String,
    /// Subscription's gateway chosen by the customer. If status is TRIALING, this field is null
    pub gateway: String,
    /// Custom fields passed (required by the product configuration)
    pub custom_fields: HashMap<String, String>,
    /// ID of the customer for which this subscription was created
    pub customer_id: String,
    /// ID of the customer (created by Stripe) for which this subscription was created
    pub stripe_customer_id: String,
    /// Your Stripe account ID
    pub stripe_account: String,
    /// ID of the Stripe subscription created for this product subscription
    pub stripe_subscription_id: String,
    /// ID of the coupon used on this subscription
    pub coupon_id: Option<String>,
    /// When this subscription period will end
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub current_period_end: SystemTime,
    /// When this subscription period will end
    pub upcoming_email_1_week_sent: bool,
    /// Email sent notifying the customer of an ending trial
    pub trial_period_ending_email_sent: bool,
    /// Whether or not a new invoice has already been created for the new period
    pub renewal_invoice_created: bool,
    /// When was this subscription created
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// When was this subscription created
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// When was this subscription canceled
    pub canceled_at: Option<String>,
    // Digital Software
    pub product_title: String,
    /// Customer name
    pub customer_name: String,
    /// Customer surname
    pub customer_surname: String,
    /// Customer phone
    pub customer_phone: Option<String>,
    /// Customer phone country code
    pub customer_phone_country_code: Option<String>,
    /// Customer country code
    pub customer_country_code: Option<String>,
    /// Customer street address
    pub customer_street_address: Option<String>,
    /// Customer street address additional info
    pub customer_additional_address_info: Option<String>,
    /// Customer city
    pub customer_city: Option<String>,
    /// Customer postal code
    pub customer_postal_code: Option<String>,
    /// Customer state
    pub customer_state: Option<String>,
    /// Customer email
    pub customer_email: String,
    /// Array of invoice objects linked to this subscription
    pub invoices: Vec<Value>,
}