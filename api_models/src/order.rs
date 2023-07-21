// Dependencies
use super::Currencies;
use crate::{
    invoice::{InvoiceStatus, InvoiceStatusDetails, InvoiceVoidDetails},
    product::ProductRaw,
    RawAPIResponse, YearMonths, WeekDays,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{formats::Flexible, TimestampSeconds};
use std::{collections::HashMap, time::SystemTime};

/// The type of orders.
/// Used in [`OrderRaw`]
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Product,
    Subscription,
    PublicRestApi,
    MonthlyBill,
    ShoppingCart,
}

/// The history of the order.
/// Used in [`OrderRaw`]
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    CustomerDisputeCancelled,
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
    pub recurring_billing_id: Option<String>,
    /// Unique ID of the recurring bill.
    pub total: f64,
    /// Total of the invoice in the product’s currency.
    pub total_display: u64,
    /// Exchange rate between currency chosen and USD.
    pub exchange_rate: f64,
    /// Exchange rate between the cryptocurrency chosen (if any) and USD.
    pub crypto_exchange_rate: f64,
    /// Available currency.
    pub currency: Currencies,
    /// The shop ID to which this invoice belongs.
    pub shop_id: u64,
    /// Deprecated.
    pub shop_image_name: Option<String>,
    /// Deprecated.
    pub shop_image_storage: Option<String>,
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
    /// Key-Value object.
    /// Key is the product ID and value is the details of the variant chosen.
    pub product_variants: Option<HashMap<String, ProductVariant>>,
    /// Unique ID of the product for which this feedback has been posted.
    pub product_id: String,
    /// Product title.
    pub product_title: String,
    /// Product type.
    pub product_type: String,
    /// Product subtype, set only with product_type `SUBSCRIPTION`.
    pub subtype: Option<String>,
    /// Field reserved for Sellix-own plans.
    /// Unique ID of the subscription purchased.
    pub subscription_id: Option<String>,
    /// Field reserved for Sellix-own plans.
    /// Time, in seconds, of the subscription purchased.
    pub subscription_time: Option<String>,
    /// Gateway chosen for this invoice.
    /// If `null`, the customer will be asked for a gateway in the Sellix hosted invoice page.
    pub gateway: String,
    /// PayPal Alternative Payment Method name, such as iDEAL, used if gateway is `PAYPAL`.
    pub paypal_apm: Option<String>,
    /// Deprecated.
    pub paypal_email: Option<String>,
    /// This field contains the PayPal order ID.
    pub paypal_order_id: Option<String>,
    /// This field is updated after the invoice is completed with the fee taken by PayPal over the invoice total.
    pub paypal_fee: Option<String>,
    /// This field is updated after the invoice is completed with the PayPal's email used for the purchase.
    pub paypal_payer_email: Option<String>,
    /// ID of the paypal subscription.
    pub paypal_subscription_id: Option<String>,
    /// Link for the merchant to purchase the subscription from.
    pub paypal_subscription_link: Option<String>,
    /// Deprecated.
    pub lex_order_id: Option<String>,
    /// Deprecated.
    pub lex_payment_method: Option<String>,
    pub paydash_payment_id: Option<String>,
    /// Client secret used to create the `STRIPE` paymentIntent.
    pub stripe_client_secret: Option<String>,
    /// If the invoice type is `PRODUCT_SUBSCRIPTION` or `SUBSCRIPTION`, refers to the `STRIPE` price ID.
    pub stripe_price_id: Option<String>,
    /// Merchant Skrill email.
    pub skrill_email: Option<String>,
    /// Skrill unique ID linked to the invoice.
    pub skrill_sid: Option<String>,
    /// Skrill link to redirect the customer to.
    pub skrill_link: Option<String>,
    /// PerfectMoney payments ID linked to the invoice.
    pub perfectmoney_id: Option<String>,
    /// Cryptocurrency address linked to this invoice.
    pub crypto_address: String,
    /// Cryptocurrency amount converted based on `crypto_exchange_rate`.
    pub crypto_amount: f64,
    /// Cryptocurrency amount received, paid by the customer.
    pub crypto_received: f64,
    /// URI used to create the QRCODE.
    pub crypto_uri: String,
    /// Crypto confirmations needed to process the invoice.
    pub crypto_confirmations_needed: u64,
    /// If true, a scheduled payout for this invoice's cryptocurrency address has been sent.
    pub crypto_scheduled_payout: bool,
    /// If true, an instant payout for this invoice's cryptocurrency address has been sent.
    pub crypto_payout: bool,
    /// If true, the Sellix `fee_percentage` has already been collected.
    pub fee_billed: bool,
    /// If invoice type is `MONTHLY_BILL`, contains a breakdown of the fees that need to be collected.
    pub bill_info: Option<Value>,
    /// Full CashApp QRCODE string.
    pub cashapp_qrcode: Option<String>,
    /// CashApp cashtag used to create the QRCODE.
    pub cashapp_cashtag: Option<String>,
    /// Unique note for the customer to leave in the CashApp payments.
    pub cashapp_note: Option<String>,
    /// Customer country.
    pub country: String,
    /// Customer location.
    pub location: String,
    /// Customer IP.
    pub ip: String,
    /// If true, a VPN or Proxy has been detected.
    pub is_vpn_or_proxy: bool,
    /// Customer User Agent.
    pub user_agent: String,
    /// Quantity of product purchased.
    pub quantity: u64,
    /// Unique ID of the coupon, if used, for the discount.
    pub coupon_id: Option<String>,
    /// key-value JSON having as key the custom field name and as value the custom field value inserted by the customer.
    /// Custom fields can both be used as inputs from the customers but also as metadata for invoices,
    /// letting you pass hidden fields for internal referencing.
    pub custom_fields: HashMap<String, String>,
    /// If `true`, this invoice has been created through the Developers API.
    pub developer_invoice: bool,
    /// If a `product_id` is not passed through the Developers API, a title must be specified.
    pub developer_title: Option<String>,
    /// Additional webhook URL to which updates regarding this invoice will be sent.
    pub developer_webhook: Option<String>,
    /// If present, the customer will be redirected to this URL after the invoice has been paid.
    pub developer_return_url: Option<String>,
    /// If present, the customer will be redirected to this URL after the invoice has been paid.
    pub status: InvoiceStatus,
    /// If `CART_PARTIAL_OUT_OF_STOCK`, the invoice has been completed but some products in the cart were out of stock.
    pub status_details: Option<InvoiceStatusDetails>,
    /// If the invoice is `VOIDED` and the product (or all the products in the cart) were out of stock,
    /// this additional status is set.
    pub void_details: Option<InvoiceVoidDetails>,
    /// If a coupon or `volume_discount` is used,
    /// the discount value presents the total amount of discount over the total cost of the invoice.
    pub discount: u64,
    /// What cut does Sellix take out of the total.
    /// To learn more about Sellix fees please refer to <https://sellix.io/fees>.
    pub fee_percentage: u64,
    /// Additional info on the customer IP.
    pub ip_info: IpInfo,
    /// If product type is `SERIALS`, provide the serials linked to this invoice.
    pub serials: Vec<String>,
    /// Can be `NULL`, info of the file linked with this invoice.
    pub file: Option<File>,
    /// If the product type is `SERVICE`, this field contains additional details on the type of service provided by the merchant.
    pub service_text: Option<String>,
    /// If the product type is `DYNAMIC`, this field contains the response from the webhook sent to get the product.
    pub dynamic_response: Option<String>,
    /// Webhook responses for this invoice.
    pub webhooks: Vec<Webhook>,
    /// Can be `NULL`, contains info about the payout transaction.
    pub crypto_payout_transaction: Option<CryptoPayoutTransaction>,
    /// Information related to the dispute (if any) opened on this order.
    pub paypal_dispute: PaypalDispute,
    /// Additional details on the invoice status change.
    pub status_history: Vec<StatusHistory>,
    /// Crypto transactions received to fulfill this invoice.
    pub crypto_transactions: Vec<CryptoTransaction>,
    /// `gateways` available for the update invoice Developers API.
    pub gateways_available: Vec<String>,
    /// If `true`, the merchant has enabled purchase with Credit Card through PayPal.
    pub shop_paypal_credit_card: bool,
    /// If `true`, the merchant has enabled invoice shipment to the PayPal customer email.
    pub shop_force_paypal_email_delivery: bool,
    /// Contains the full product object for this feedback.
    pub product: ProductRaw,
    /// Deprecated.
    pub day_value: u64,
    /// Deprecated.
    pub day: WeekDays,
    /// Deprecated.
    pub month: YearMonths,
    /// Deprecated.
    pub year: u64,
    /// Creation date of the order.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// Date, available if the blacklist has been edited.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// User ID, available if the blacklist has been edited.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_by: SystemTime,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#order-get>.
/// Used in [`OrderGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderOneRaw {
    pub order: OrderRaw,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#order-get>.
pub type OrderGetResponseRaw = RawAPIResponse<OrderOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#order-list>.
/// Used for [`OrderListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderArray {
    pub orders: Vec<OrderRaw>,
}
/// Raw API response from here.
/// <https://developers.sellix.io/#order-list>.
pub type OrderListResponseRaw = RawAPIResponse<OrderArray>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductVariant {
    pub price: f64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpInfo {
    pub success: bool,
    pub message: String,
    pub fraud_score: u64,
    pub country_code: String,
    pub region: String,
    pub city: String,
    pub isp: String,
    pub asn: u64,
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
    pub latitude: u64,
    pub longitude: u64,
    pub request_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct File {
    pub id: u64,
    pub uniqid: String,
    pub cloudflare_image_id: String,
    pub storage: Option<String>,
    pub name: String,
    pub original_name: String,
    pub extension: String,
    pub shop_id: u64,
    pub size: u64,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct Webhook {
    pub uniqid: String,
    pub url: String,
    pub event: String,
    pub retries: u64,
    pub response_code: u64,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    pub payload: String,
    pub response: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct CryptoPayoutTransaction {
    pub to_address: String,
    pub from_address: String,
    pub crypto_amount: f64,
    pub hash: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct PaypalDispute {
    pub id: String,
    pub invoice_id: String,
    pub shop_id: u64,
    pub reason: String,
    pub status: String,
    pub outcome: Option<String>,
    pub messages: Vec<Message>,
    pub life_cycle_stage: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub seller_response_due_date: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct Message {
    pub posted_by: String,
    pub content: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct StatusHistory {
    pub id: u64,
    pub invoice_id: String,
    pub status: String,
    pub details: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct CryptoTransaction {
    pub crypto_amount: f64,
    pub hash: String,
    pub confirmations: u64,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeDiscount {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: u64,
    pub quantity: u64,
}