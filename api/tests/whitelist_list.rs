// Dependencies
use dotenv::dotenv;
use sellix_rs::Client;
use std::env;

// Entrypoint
#[tokio::test]
async fn whitelist_list() {
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

    // Grab all of the whitelists
    let whitelists = whitelist_client.get_list(Some(0)).await;
    assert!(whitelists.is_ok(), "unable to list whitelists");
}