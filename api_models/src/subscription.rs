// Dependencies
use crate::UniqidDict;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use std::time::SystemTime;
use super::RawAPIResponse;
use super::payment::PaymentGateway;

/// All of the possible intervals subscriptions can be billed within
/// <https://developers.sellix.io/#subscriptions-features>.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum RecurringBillingIntervals {
    Daily,
    Weekly,
    Monthly,
    Yearly
}

/// All of the possible statuses a subscription can be in
/// <https://developers.sellix.io/#subscriptions-configure-webhooks>.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum SubscriptionStatus {
    Pending,
    Cancelled,
    Trialing,
    Active
}

/// Represents the possible customer details from an API object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
/// <https://developers.sellix.io/#subscriptions-configure-webhooks>.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionRaw {
    /// ID of the subscription (recurring bill)
    pub id: String,
    /// ID of the shop whose resource is assigned to
    pub shop_id: u64,
    /// ID of the shop whose resource is assigned to
    pub product_id: String,
    /// Subscription's status, see more info about statuses below
    pub status: SubscriptionStatus,
    /// Subscription's gateway chosen by the customer. If status is TRIALING, this field is null
    pub gateway: PaymentGateway,
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

/// Raw API response from here.
/// <https://developers.sellix.io/#subscriptions-get>.
/// Used in [`SubscriptionGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionOneRaw {
    pub subscription: SubscriptionRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#subscriptions-get>.
pub type SubscriptionGetResponseRaw = RawAPIResponse<SubscriptionOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#subscriptions-list>.
/// Used for [`SubscriptionListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionArray {
    pub subscriptions: Vec<SubscriptionRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#subscriptions-list>.
pub type SubscriptionListResponseRaw = RawAPIResponse<SubscriptionArray>;

/// Represents the payload for creating a subscription.
/// <https://developers.sellix.io/#subscriptions-create>.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionCreatePayload<'a> {
    /// ID of the subscription product.
    product_id: &'a str,
    /// ID of the subscription product.
    coupon_code: Option<&'a str>,
    /// key-value JSON having as key the custom field name and as value the custom field value inserted by the customer. Custom fields can both be used as inputs from the customers but also as metadata for invoices, letting you pass hidden fields for internal referencing.
    custom_fields: HashMap<String, String>,
    /// ID of the store customer.
    customer_id: &'a str,
    gateway: PaymentGateway
}

/// Represents the response after creating a subscription.
/// <https://developers.sellix.io/#subscriptions-create>.
pub type SubscriptionCreateResponseRaw = RawAPIResponse<UniqidDict>;