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

    // Build the client that would send out requests to the blacklist API
    let client = Client::new( api_key, merchant );
    let blacklist_client = client.blacklist;

    // Grab all of the blacklists
    let blacklists = blacklist_client.get_list(Some(0)).await;
    if let Err(err) = blacklists {
        println!("error: unable to list blacklists - {}", err);
    } else if let Ok(all) = blacklists {
        println!("recieved {} blacklists", all.blacklists.len())
    }
}