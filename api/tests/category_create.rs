// Dependencies
use dotenv::dotenv;
use sellix_rs::{Client, sellix_api_models::category::CategoryCreatePayload};
use std::env;

// Entrypoint
#[tokio::test]
async fn category_create() {
    dotenv().ok();

    // Grab our environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not provided");
    let merchant = env::var("MERCHANT").ok();

    // Warn if we do not have a merchant
    if merchant.is_none() {
        println!("warning: MERCHANT environment variable is not present. Defaulting to first merchant.");
    }

    // Build the client that would send out requests to the category API
    let client = Client::new( &api_key, merchant.as_deref() );
    let category_client = client.category;

    // Create a category
    let category = category_client.create(CategoryCreatePayload {
        title: "Software",
        unlisted: Some(false),
        products_bound: None,
        groups_array: None,
        sort_priority: None
    }).await;
    assert!(category.is_ok(), "unable to create categories");
    std::env::set_var("CATEGORY_UNIQID", category.unwrap().uniqid);
}