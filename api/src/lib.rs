/*
Exports
*/
pub mod models;
pub mod api;

// Dependencies
use sha2::Sha512;
use hmac::{Hmac, Mac};

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