// Dependencies
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use std::time::SystemTime;
use super::{WeekDays, YearMonths};

/// Represents a product, only by its id and quantity.
/// Used for [`SellixPaymentPayload`].
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPayment {
    uniqid: String,
    unit_quantity: u64,
}

/// Represents a cart for the list of products when using Sellix Pay.
/// <https://developers.sellix.io/#sellix_checkout>
#[derive(Debug, Serialize, Deserialize)]
pub struct Cart {
    products: Vec<ProductPayment>,
}

/// States how risky a customer is.
#[derive(Debug, Serialize, Deserialize)]
pub struct FraudShield {
    ip: String,
    user_agent: String,
    user_language: String,
}

/// All of the payment gateways.
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
enum PaymentGateways {
    Paypal,
    Ethereum,
    BinanceCoin,
    Bitcoin,
    BitcoinCash,
    Litecoin,
    Skrill,
    Stripe,
    PerfectMoney,
    CashApp,
    LexHoldingsGroup,
    Paydash,
    Monero,
    Concordium,
    BitcoinLn,
    Nano,
    Solana,
    Ripple,
    Usdt,
    Usdc,
    Plz,
    Polygon,
    Tron,
    Binance
}

/// All of the possible payment methods.
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="lowercase")]
pub enum PaymentMethod {
    Bancontact,
    Eps,
    Trustly,
    Mercado,
    Paylater,
    Sepa,
    Venmo,
    Blik,
    Giropay,
    Ideal,
    Mybank,
    Sofort,
    Przelewy24,
    Credit
}

/// Represents a Sellix Pay payload.
/// <https://developers.sellix.io/#sellix_checkout>
#[derive(Debug, Serialize, Deserialize)]
pub struct SellixPaymentPayload {
    /// Required if `product_id` and `cart` are null.
    /// Defines the title of the purchase, can be the digital good's name or a brief summary of what the customer is paying for.
    title: String,
    /// If specified `value`, `currency` and `custom_fields` will be taken from the product details.
    product_id: Option<String>,
    /// Other than `product_id`, a [`Cart`] object can be specified if multiple products need to be purchased through the API.
    cart: Cart,
    /// If null, the customer will be asked automatically to choose a gateway on the Sellix hosted /payment page.
    /// If `product_id` is specified, the gateway must be on in the product's gateways array.
    gateway: Option<PaymentGateways>,
    /// Array of accepted gateways, if gateway is NULL the user will be prompted to choose one of these.
    gateways: Vec<PaymentGateways>,
    /// Object containing product addons available for the product.
    /// Key-value with Key being the product ID and Value a comma separated list of addon IDs.
    /// This can be used on cart payments as well.
    product_addons: HashMap<String, String>,
    /// Object containing product variants available for the product.
    /// Key-value with Key being the product id and Value the variant title (case sensitive).
    /// This can be used on cart payments as well.
    product_variants: HashMap<String, String>,
    /// If gateway is PAYPAL, a `paypal_apm` (PayPal Alternative Payment Method) can be specified. 
    /// To retrieve the available PayPal APM for a specific customer session,
    /// please refer to the PayPal SDK using `window.paypal.FUNDING` and `fundingSource` to filter out available methods. 
    /// You can also use our documentation on how to process white_label payments.
    paypal_apm: Option<PaymentGateways>,
    /// If `gateway` is `PAYPAL` and no `paypal_apm` is passed,
    /// specify `credit_card` `true` to land the customer on the PayPal managed credit card page instead of the onboarding login.
    credit_card: Option<String>,
    /// Deprecated
    lex_payment_method: Option<String>,
    /// Required if `product_id` and `cart` are `null`.
    /// The customer will be asked to pay for this amount.
    value: f32,
    /// Required if `product_id` and `cart` are null.
    /// The customer will be asked to pay for this amount.
    currency: String,
    /// Can be passed with either `product_id` `null` or not.
    /// The `value` or product's `price` will be multiplied by this amount.
    quantity: u64,
    /// Pass a Sellix coupon code to apply a discount over the invoice.
    coupon_code: String,
    /// Cryptocurrency confirmations required to count a transaction over the total crypto amount needed.
    confirmations: u64,
    /// Email of the customer.
    /// Should you want to handle emails on your own,
    /// pass to this field a company email to which PDF receipts of orders will be sent for accounting and log purposes.
    email: String,
    /// key-value JSON having as key the custom field name and as value the custom field value inserted by the customer.
    /// Custom fields can both be used as inputs from the customers but also as metadata for invoices,
    /// letting you pass hidden fields for internal referencing.
    custom_fields: HashMap<String, String>,
    /// Customer details to be used by our fraud shield in order to score potential fraud attempts.
    fraud_shield: FraudShield,
    /// Webhook URL to which updates regarding this payment (invoice) will be sent.
    webhook: String,
    /// Whether or not you want to handle the payments UI.
    /// If `false`, `return_url` must be specified as it is the website where we will redirect the customer once he has paid through our platform.
    /// If `true`, we will return a full `invoice` object in the response for you to handle.
    /// You can receive updates over invoices and handle subsequent logics through our webhooks.
    white_label: bool,
    /// Return url, required if `white_label` is `false`.
    return_url: String,
}

/// Represents the response from a successful Sellix Pay payment creation.
/// <https://developers.sellix.io/#sellix_checkout>.
#[derive(Debug, Serialize, Deserialize)]
pub struct SellixPaymentResponse {
    url: String,
    uniqid: String
}

/// Represents a product, only by its title, price and currency.
/// Used for [`SellixPaymentResponseWL`].
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPaymentResponse {
    title: String,
    price_display: f32,
    currency: String
}

/// Represents the response from a successful Sellix Pay payment creation, as white label.
/// <https://developers.sellix.io/#sellix_checkout>.
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct SellixPaymentResponseWL {
    id: u64,
    uniqid: String,
    total: f32,
    total_display: f32,
    currency: String,
    exchange_rate: f32,
    crypto_exchange_rate: f32,
    shop_id: u64,
    name: String,
    customer_email: String,
    product_id: String,
    product_type: String,
    product_price: f32,
    file_attachment_uniqid: Option<String>,
    gateway: String,
    paypal_email: Option<String>,
    paypal_order_id: Option<String>,
    paypal_payer_email: Option<String>,
    skrill_email: Option<String>,
    skrill_sid: Option<String>,
    skrill_link: Option<String>,
    perfectmoney_id: Option<String>,
    crypto_address: String,
    crypto_amount: f32,
    crypto_received: f32,
    crypto_uri: String,
    crypto_confirmations_needed: u64,
    country: String,
    location: String,
    ip: String,
    is_vpn_or_proxy: bool,
    user_agent: String,
    quantity: u64,
    coupon_id: Option<String>,
    custom_fields: HashMap<String, String>,
    developer_invoice: bool,
    developer_title: String,
    developer_webhook: String,
    developer_return_url: String,
    status: String,
    discount: f32,
    fee_fixed: f32,
    fee_percentage: f32,
    day_value: f32,
    day: WeekDays,
    month: YearMonths,
    year: u64,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    created_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    updated_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    updated_by: SystemTime,
    serials: Vec<String>,
    file: Option<String>,
    webhooks: Vec<String>,
    crypto_payout: bool,
    crypto_payout_transaction: Option<String>,
    crypto_transactions: Vec<String>,
    product: ProductPaymentResponse,
    total_conversions: HashMap<String, f32>,
    theme: String
}