/*
Exports
*/
pub mod models;
pub mod api;
pub mod client;
use std::collections::HashMap;

pub use client::Client;

// Dependencies
use models::payment::PaymentGateway;
use serde_repr::{Serialize_repr, Deserialize_repr};
use sha2::Sha512;
use hmac::{Hmac, Mac};
use strum_macros::FromRepr;

// Create alias from HMAC-SHA512
type HmacSha512 = Hmac<Sha512>;

/// Verifies the authenticity of a webhook request and its payload.
/// The body must be a JSON-string.
pub fn verify_signature(webhook_secret: &str, signature: &str, body: &str) -> bool {
    // Make sure we are given a webhook secret
    let decoded_signature = hex::decode(signature).expect("invalid signature - malformed hex");

    // Create the Hmac and feed it the body
    let mut mac = HmacSha512::new_from_slice(webhook_secret.as_bytes())
        .expect("unable to initialise hmac");
    mac.update(body.as_bytes());

    // Verify
    mac.verify_slice(&decoded_signature).is_ok()
}

/// Used in [`ProductQuerystringBuilder`]
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr, PartialEq, FromRepr)]
#[repr(u16)]
pub enum QueryStep {
    /// The default checkout step (product details).
    Default = 0,
    // The gateway choice step.
    Choice = 1,
    /// The email and pay button step.
    EmailAndPay = 3
}

/// Allows merchants can share links to their products on their Sellix website, similar to this example:
/// 
/// https://exampleStore.mysellix.io/product/demobde8a50
/// 
/// These links can have querystring parameters added to them, like this example:
/// 
/// https://exampleStore.mysellix.io/product/demode8a50?quantity=5&gateway=LITECOIN&step=1
pub struct ProductQuerystringBuilder {
    store_name: String,
    uniqid: String,
    /// Implement any more custom fields.
    pub custom_fields: HashMap<String, String>,
    /// Specifies the number of the same product to purchase.
    quantity: Option<u64>,
    /// Specifies which payment gateway should be selected.
    gateway: Option<PaymentGateway>,
    /// Specifies which step of the checkout process should be displayed.
    step: Option<QueryStep>,
    /// The customer's email address.
    email: Option<String>,
    /// The code of a coupon that should be applied automatically to the purchase.
    coupon_code: Option<String>,
}
impl ProductQuerystringBuilder {
    /// Initialises a builder.
    pub fn new(store_name: &str, uniqid: &str) -> Self {
        Self {
            store_name: store_name.to_owned(),
            uniqid: uniqid.to_owned(),
            custom_fields: HashMap::default(),
            quantity: None,
            gateway: None,
            step: None,
            email: None,
            coupon_code: None,
        }
    }

    /// Sets the quantity.
    pub fn quantity(mut self, quantity: u64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Sets the gateway.
    pub fn gateway(mut self, gateway: PaymentGateway) -> Self {
        self.gateway = Some(gateway);
        self
    }

    /// Sets the step.
    pub fn step(mut self, step: QueryStep) -> Self {
        self.step = Some(step);
        self
    }

    /// Sets the quantity.
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }

    /// Sets the quantity.
    pub fn coupon_code(mut self, coupon_code: &str) -> Self {
        self.coupon_code = Some(coupon_code.to_owned());
        self
    }

    /// Add a custom field.
    pub fn add_custom_field(mut self, key: &str, value: &str) -> Self {
        self.custom_fields.insert(key.to_owned(), value.to_owned());
        self
    }

    /// Builds the string to a URL.
    pub fn build(&self) -> String {
        // Create the base string
        let mut output = format!("https://{}.mysellix.io/product/{}?", self.store_name, self.uniqid);

        // Add all optional fields to the string
        if let Some(quantity) = self.quantity  {
            output.push_str(&format!("quantity={}&", quantity));
        }
        if let Some(gateway) = &self.gateway  {
            output.push_str(&format!("gateway={}&", gateway));
        }
        if let Some(step) = &self.step  {
            output.push_str(&format!("step={}&", serde_json::to_string(step).unwrap()));
        }
        if let Some(email) = &self.email  {
            output.push_str(&format!("email={}&", email));
        }
        if let Some(coupon_code) = &self.coupon_code  {
            output.push_str(&format!("couponCode={}&", coupon_code));
        }

        // Add all custom fields
        for (name, value) in &self.custom_fields {
            output.push_str(&(name.to_owned() + "=" + &value + "&"));
        }

        // Return output
        output.pop();
        output
    }
}