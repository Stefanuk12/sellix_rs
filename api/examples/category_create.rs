// Dependencies
use dotenv::dotenv;
use sellix_rs::{ models::category::CategoryCreatePayload, Client };
use std::env;

// Entrypoint
#[tokio::main]
async fn main() {
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

    // Grab all of the categories
    let categorys = category_client.create(CategoryCreatePayload {
        title: "Software",
        unlisted: Some(false),
        products_bound: None,
        groups_array: None,
        sort_priority: None
    }).await;
    if let Err(err) = categorys {
        println!("error: unable to create categories - {}", err);
    } else if let Ok(created) = categorys {
        println!("created category with uniqid {}", created.uniqid);
    }
}