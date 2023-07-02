// Dependencies
use dotenv::dotenv;
use sellix_rs::Client;
use std::env;

// Entrypoint
#[tokio::test]
async fn blacklist_remove() {
    dotenv().ok();

    // Grab our environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not provided");
    let merchant = env::var("MERCHANT").ok();
    let blacklist_uniqid = env::var("BLACKLIST_UNIQID").expect("BLACKLIST_UNIQID not provided");

    // Warn if we do not have a merchant
    if merchant.is_none() {
        println!("warning: MERCHANT environment variable is not present. Defaulting to first merchant.");
    }

    // Build the client that would send out requests to the blacklist API
    let client = Client::new( &api_key, merchant.as_deref() );
    let blacklist_client = client.blacklist;

    // Remove the blacklist
    let blacklist = blacklist_client.delete(&blacklist_uniqid).await;
    assert!(blacklist.is_ok(), "unable to remove blacklist");
}