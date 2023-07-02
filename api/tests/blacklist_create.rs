// Dependencies
use dotenv::dotenv;
use sellix_rs::{Client, sellix_api_models::blacklist::{BlacklistCreatePayload, BlacklistTypes}};
use std::env;

// Entrypoint
#[tokio::test]
async fn blacklist_create() {
    dotenv().ok();

    // Grab our environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not provided");
    let merchant = env::var("MERCHANT").ok();

    // Warn if we do not have a merchant
    if merchant.is_none() {
        println!("warning: MERCHANT environment variable is not present. Defaulting to first merchant.");
    }

    // Build the client that would send out requests to the blacklist API
    let client = Client::new( &api_key, merchant.as_deref() );
    let blacklist_client = client.blacklist;

    // Create a blacklist
    let blacklist = blacklist_client.create(BlacklistCreatePayload {
        r#type: BlacklistTypes::Email,
        data: "test@example.com",
        note: Some("Testing")
    }).await;
    assert!(blacklist.is_ok(), "unable to create blacklist");
    std::env::set_var("BLACKLIST_UNIQID", blacklist.unwrap().uniqid);
}