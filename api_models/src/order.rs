// Dependencies
use serde::{Serialize, Deserialize};
use serde_json::Value;
use super::Currencies;

/// The type of orders.
/// Used in [`OrderRaw`]
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Product,
    Subscription,
    PublicRestApi,
    MonthlyBill,
    ShoppingCart
}

/// The history of the order.
/// Used in [`OrderRaw`]
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum OrderHistoryStatus {
    Refunded,
    PaymentCaptureCompleted,
    PaymentCaptureDenied,
    Partial,
    CustomerDisputeResolved,
    Completed,
    PaymentCaptureReversed,
    Voided,
    PaymentCapturePending,
    WaitingForConfirmations,
    PaymentAuthorizationCreated,
    CustomerDisputeOngoing,
    CheckoutOrderApproved,
    PaymentAuthorizationVoided,
    CheckoutOrderCompleted,
    CustomerDisputeUpdated,
    CustomerDisputeCancelled
}

/// Represents the raw API response for a order object.
/// <https://developers.sellix.io/#orders>.
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRaw {
    /// ID of the resource.
    pub id: u64,
    /// Unique ID of the resource, used as reference across the API.
    pub uniqid: String,
    /// Unique ID of the recurring bill.
    pub recurring_billing_id: Value,
    /// Unique ID of the recurring bill.
    pub total: f64,
    /// Total of the invoice in the product’s currency.
    pub total_display: i64,
    /// Exchange rate between currency chosen and USD.
    pub exchange_rate: f64,
    /// Exchange rate between the cryptocurrency chosen (if any) and USD.
    pub crypto_exchange_rate: f64,
    /// Available currency.
    pub currency: Currencies,
    /// The shop ID to which this invoice belongs.
    pub shop_id: i64,
    /// Deprecated.
    pub shop_image_name: Value,
    /// Deprecated.
    pub shop_image_storage: Value,
    /// New field containing the cloudflare image ID of this product,
    /// replaces image_attachment and image_name.
    /// 
    /// Format https://imagedelivery.net/95QNzrEeP7RU5l5WdbyrKw/
    /// where
    /// 
    /// `variant_name` can be `shopItem`, `avatar`, `icon`, `imageAvatarFeedback`, `public`, `productImageCart`.
    pub cloudflare_image_id: String,
    /// Name of the merchant.
    pub name: String,
    /// Invoice type.
    /// For subscriptions, the invoice type is `PRODUCT_SUBSCRIPTION`
    /// as `SUBSCRIPTION` is reserved for Sellix-own plans.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Email of the customer.
    pub customer_email: String,
    /// If `true` and gateway is `PAYPAL`,
    /// the product will be shipped to the PayPal email on record instead of the customer email.
    pub paypal_email_delivery: bool,
    pub product_variants: ProductVariants,
    /// Unique ID of the product for which this feedback has been posted.
    pub product_id: String,
    /// Product title.
    pub product_title: String,
    /// Product type.
    pub product_type: String,
    /// Product subtype, set only with product_type `SUBSCRIPTION`.
    pub subtype: Value,
    /// Field reserved for Sellix-own plans.
    /// Unique ID of the subscription purchased.
    pub subscription_id: Value,
    /// Field reserved for Sellix-own plans.
    /// Time, in seconds, of the subscription purchased.
    pub subscription_time: Value,
    /// Gateway chosen for this invoice.
    /// If `null`, the customer will be asked for a gateway in the Sellix hosted invoice page.
    pub gateway: String,
    pub paypal_apm: Value,
    pub paypal_email: Value,
    pub paypal_order_id: Value,
    pub paypal_fee: Value,
    pub paypal_payer_email: Value,
    pub paypal_subscription_id: Value,
    pub paypal_subscription_link: Value,
    pub lex_order_id: Value,
    pub lex_payment_method: Value,
    pub paydash_payment_id: Value,
    pub stripe_client_secret: Value,
    pub stripe_price_id: Value,
    pub skrill_email: Value,
    pub skrill_sid: Value,
    pub skrill_link: Value,
    pub perfectmoney_id: Value,
    pub crypto_address: String,
    pub crypto_amount: f64,
    pub crypto_received: f64,
    pub crypto_uri: String,
    pub crypto_confirmations_needed: i64,
    pub crypto_scheduled_payout: bool,
    pub crypto_payout: bool,
    pub fee_billed: bool,
    pub bill_info: Value,
    pub cashapp_qrcode: Value,
    pub cashapp_cashtag: Value,
    pub cashapp_note: Value,
    pub country: String,
    pub location: String,
    pub ip: String,
    pub is_vpn_or_proxy: bool,
    pub user_agent: String,
    pub quantity: i64,
    pub coupon_id: Value,
    pub custom_fields: CustomFields,
    pub developer_invoice: bool,
    pub developer_title: Value,
    pub developer_webhook: Value,
    pub developer_return_url: Value,
    pub status: String,
    pub status_details: Value,
    pub void_details: Value,
    pub discount: i64,
    pub fee_percentage: i64,
    pub ip_info: IpInfo,
    pub serials: Vec<String>,
    pub file: File,
    pub service_text: Value,
    pub dynamic_response: Value,
    pub webhooks: Vec<Webhook>,
    pub crypto_payout_transaction: CryptoPayoutTransaction,
    pub paypal_dispute: PaypalDispute,
    pub status_history: Vec<StatusHistory>,
    pub crypto_transactions: Vec<CryptoTransaction>,
    pub gateways_available: Vec<String>,
    pub shop_paypal_credit_card: bool,
    pub shop_force_paypal_email_delivery: bool,
    pub product: Product,
    pub day_value: i64,
    pub day: String,
    pub month: String,
    pub year: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub updated_by: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductVariants {
    pub n6300ae0a660b0: n6300ae0a660b0,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct n6300ae0a660b0 {
    pub price: i64,
    pub title: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomFields {
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpInfo {
    pub success: bool,
    pub message: String,
    pub fraud_score: i64,
    pub country_code: String,
    pub region: String,
    pub city: String,
    pub isp: String,
    pub asn: i64,
    pub operating_system: String,
    pub browser: String,
    pub organization: String,
    pub is_crawler: bool,
    pub timezone: String,
    pub mobile: bool,
    pub host: String,
    pub proxy: bool,
    pub vpn: bool,
    pub tor: bool,
    pub active_vpn: bool,
    pub active_tor: bool,
    pub device_brand: String,
    pub device_model: String,
    pub recent_abuse: bool,
    pub bot_status: bool,
    pub connection_type: String,
    pub abuse_velocity: String,
    pub zip_code: String,
    pub latitude: i64,
    pub longitude: i64,
    pub request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub uniqid: String,
    pub cloudflare_image_id: String,
    pub storage: Value,
    pub name: String,
    pub original_name: String,
    pub extension: String,
    pub shop_id: i64,
    pub size: i64,
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    pub uniqid: String,
    pub url: String,
    pub event: String,
    pub retries: i64,
    pub response_code: i64,
    pub created_at: i64,
    pub payload: String,
    pub response: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoPayoutTransaction {
    pub to_address: String,
    pub from_address: String,
    pub crypto_amount: f64,
    pub hash: String,
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaypalDispute {
    pub id: String,
    pub invoice_id: String,
    pub shop_id: i64,
    pub reason: String,
    pub status: String,
    pub outcome: Value,
    pub messages: Vec<Message>,
    pub life_cycle_stage: String,
    pub seller_response_due_date: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub posted_by: String,
    pub content: String,
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusHistory {
    pub id: i64,
    pub invoice_id: String,
    pub status: String,
    pub details: String,
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoTransaction {
    pub crypto_amount: f64,
    pub hash: String,
    pub confirmations: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub uniqid: String,
    pub shop_id: i64,
    pub type_field: String,
    pub subtype: Value,
    pub title: String,
    pub currency: String,
    pub price: i64,
    pub price_display: f64,
    pub description: String,
    pub image_attachment: Value,
    pub file_attachment: Value,
    pub volume_discounts: Vec<VolumeDiscount>,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
    pub trial_period: i64,
    pub paypal_product_id: Value,
    pub paypal_plan_id: Value,
    pub stripe_price_id: String,
    pub quantity_min: i64,
    pub quantity_max: i64,
    pub quantity_warning: i64,
    pub gateways: Vec<String>,
    pub custom_fields: Vec<CustomField>,
    pub crypto_confirmations_needed: i64,
    pub max_risk_level: i64,
    pub block_vpn_proxies: bool,
    pub delivery_text: String,
    pub service_text: String,
    pub stock_delimiter: String,
    pub stock: i64,
    pub dynamic_webhook: Value,
    pub sort_priority: i64,
    pub unlisted: bool,
    pub on_hold: bool,
    pub terms_of_service: Value,
    pub warranty: i64,
    pub warranty_text: String,
    pub private: bool,
    pub name: String,
    pub image_name: Value,
    pub image_storage: Value,
    pub cloudflare_image_id: String,
    pub serials: Vec<Value>,
    pub webhooks: Vec<Value>,
    pub feedback: Feedback,
    pub theme: String,
    pub dark_mode: i64,
    pub average_score: f64,
    pub sold_count: i64,
    pub lex_payment_methods: Vec<Value>,
    pub created_at: i64,
    pub updated_at: i64,
    pub updated_by: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeDiscount {
    pub type_field: String,
    pub value: i64,
    pub quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomField {
    pub type_field: String,
    pub name: String,
    pub regex: Value,
    pub placeholder: Value,
    pub default: Value,
    pub required: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    pub total: i64,
    pub positive: i64,
    pub neutral: i64,
    pub negative: i64,
}