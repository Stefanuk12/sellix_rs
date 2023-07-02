// Dependencies
use dotenv::dotenv;
use sellix_rs::{Client, sellix_api_models::whitelist::{WhitelistCreatePayload, WhitelistTypes}};
use std::env;

// Entrypoint
#[tokio::test]
async fn whitelist_update() {
    dotenv().ok();

    // Grab our environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not provided");
    let merchant = env::var("MERCHANT").ok();
    let whitelist_uniqid = env::var("WHITELIST_UNIQID").expect("WHITELIST_UNIQID not provided");

    // Warn if we do not have a merchant
    if merchant.is_none() {
        println!("warning: MERCHANT environment variable is not present. Defaulting to first merchant.");
    }

    // Build the client that would send out requests to the whitelist API
    let client = Client::new( &api_key, merchant.as_deref() );
    let whitelist_client = client.whitelist;

    // Update the whitelist
    let whitelist = whitelist_client.edit(&whitelist_uniqid, WhitelistCreatePayload {
        r#type: WhitelistTypes::Email,
        data: "test2@example.com",
        note: Some("Testing")
    }).await;
    assert!(whitelist.is_ok(), "unable to edit whitelists");
}