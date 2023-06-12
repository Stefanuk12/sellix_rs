// Dependencies
use dotenv::dotenv;
use sellix_rs::api::Blacklist;
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

    // Build the client that would send out requests to the blacklist API
    let blacklist_client = Blacklist {
        api_key,
        merchant
    };

    // Grab all of the blacklists
    let blacklists = blacklist_client.delete("64878b9f5af5d").await;
    if let Err(err) = blacklists {
        println!("error: unable to remove blacklist - {}", err);
    } else if let Ok(all) = blacklists {
        println!("blacklist removed: {}", all);
    }
}