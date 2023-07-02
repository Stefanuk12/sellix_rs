// Dependencies
use super::category::Feedback;
use super::order::VolumeDiscount;
use super::payment::PaymentGateway;
use super::subscription::RecurringBillingIntervals;
use super::Currencies;
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;
use std::time::SystemTime;

/// Types of products.
/// Used in [`ProductRaw`].
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    Serials,
    File,
    Service,
    Dynamic,
    InfoCard,
    Subscription,
}

/// Types of sub products.
/// Used in [`ProductRaw`].
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductSubType {
    Serials,
    File,
    Service,
    Dynamic,
}

/// Used in [`CategoryRaw`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomField {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub regex: Option<String>,
    pub placeholder: Option<String>,
    pub default: Option<String>,
    pub required: bool,
}

/// Used in [`CategoryRaw`].
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductRaw {
    pub id: u64,
    pub uniqid: String,
    pub shop_id: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub subtype: Option<String>,
    pub title: String,
    pub currency: Currencies,
    pub price: f64,
    pub price_display: f64,
    pub description: String,
    pub image_attachment: Option<String>,
    pub file_attachment: Option<String>,
    pub volume_discounts: Vec<VolumeDiscount>,
    pub recurring_interval: RecurringBillingIntervals,
    pub recurring_interval_count: u64,
    pub trial_period: u64,
    pub paypal_product_id: Option<String>,
    pub paypal_plan_id: Option<String>,
    pub stripe_price_id: String,
    pub quantity_min: i32,
    pub quantity_max: i32,
    pub quantity_warning: u64,
    pub gateways: Vec<PaymentGateway>,
    pub custom_fields: Vec<CustomField>,
    pub crypto_confirmations_needed: u64,
    pub max_risk_level: u64,
    pub block_vpn_proxies: bool,
    pub delivery_text: String,
    pub service_text: String,
    pub stock_delimiter: String,
    pub stock: i32,
    pub dynamic_webhook: Option<String>,
    pub sort_priority: u64,
    pub unlisted: bool,
    pub on_hold: bool,
    pub terms_of_service: Option<String>,
    pub warranty: u64,
    pub warranty_text: String,
    pub private: bool,
    pub name: String,
    pub image_name: Option<String>,
    pub image_storage: Option<String>,
    pub cloudflare_image_id: String,
    pub serials: Vec<String>,
    pub webhooks: Vec<String>,
    pub feedback: Feedback,
    pub theme: String,
    pub dark_mode: u64,
    pub average_score: f64,
    pub sold_count: u64,
    pub lex_payment_methods: Vec<String>,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    pub updated_by: u64,
}
