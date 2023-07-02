// Dependencies
use dotenv::dotenv;
use sellix_rs::{Client, sellix_api_models::whitelist::{WhitelistCreatePayload, WhitelistTypes}};
use std::env;

// Entrypoint
#[tokio::test]
async fn whitelist_create() {
    dotenv().ok();

    // Grab our environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not provided");
    let merchant = env::var("MERCHANT").ok();

    // Warn if we do not have a merchant
    if merchant.is_none() {
        println!("warning: MERCHANT environment variable is not present. Defaulting to first merchant.");
    }

    // Build the client that would send out requests to the whitelist API
    let client = Client::new( &api_key, merchant.as_deref() );
    let whitelist_client = client.whitelist;

    // Create a whitelist
    let whitelist = whitelist_client.create(WhitelistCreatePayload {
        r#type: WhitelistTypes::Email,
        data: "test@example.com",
        note: Some("Testing")
    }).await;
    assert!(whitelist.is_ok(), "unable to create whitelist");
    std::env::set_var("WHITELIST_UNIQID", whitelist.unwrap().uniqid);
}