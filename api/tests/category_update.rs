// Dependencies
use dotenv::dotenv;
use sellix_rs::{Client, sellix_api_models::category::CategoryCreatePayload};
use std::env;

// Entrypoint
#[tokio::test]
async fn category_update() {
    dotenv().ok();

    // Grab our environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not provided");
    let merchant = env::var("MERCHANT").ok();
    let category_uniqid = env::var("CATEGORY_UNIQID").expect("CATEGORY_UNIQID not provided");

    // Warn if we do not have a merchant
    if merchant.is_none() {
        println!("warning: MERCHANT environment variable is not present. Defaulting to first merchant.");
    }

    // Build the client that would send out requests to the category API
    let client = Client::new( &api_key, merchant.as_deref() );
    let category_client = client.category;

    // Edit the category
    let category = category_client.edit(&category_uniqid, CategoryCreatePayload {
        title: "Software",
        unlisted: Some(false),
        products_bound: None,
        groups_array: None,
        sort_priority: None
    }).await;
    assert!(category.is_ok(), "unable to edit category");
}