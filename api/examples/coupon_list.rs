// Dependencies
use dotenv::dotenv;
use sellix_rs::Client;
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

    // Build the client that would send out requests to the coupon API
    let client = Client::new( &api_key, merchant.as_deref() );
    let coupon_client = client.coupon;

    // Grab all of the coupons
    let coupons = coupon_client.get_list(Some(0)).await;
    if let Err(err) = coupons {
        println!("error: unable to list coupons - {}", err);
    } else if let Ok(all) = coupons {
        println!("recieved {} coupons", all.coupons.len())
    }
}