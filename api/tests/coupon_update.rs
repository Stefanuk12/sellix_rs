// Dependencies
use dotenv::dotenv;
use sellix_rs::{Client, sellix_api_models::coupon::CouponCreatePayload};
use std::env;

// Entrypoint
#[tokio::test]
async fn coupon_update() {
    dotenv().ok();

    // Grab our environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not provided");
    let merchant = env::var("MERCHANT").ok();
    let coupon_uniqid = env::var("COUPON_UNIQID").expect("COUPON_UNIQID not provided");

    // Warn if we do not have a merchant
    if merchant.is_none() {
        println!("warning: MERCHANT environment variable is not present. Defaulting to first merchant.");
    }

    // Build the client that would send out requests to the coupon API
    let client = Client::new( &api_key, merchant.as_deref() );
    let coupon_client = client.coupon;

    // Update the coupon
    let coupon = coupon_client.edit(&coupon_uniqid, CouponCreatePayload {
        code: "test_coupon",
        discount_value: 3,
        max_uses: None,
        products_bound: None,
        discount_type: None,
        discount_order_type: None,
        disabled_with_volume_discounts: None,
        all_recurring_bill_invoices: None,
        expire_at: None
    }).await;
    assert!(coupon.is_ok(), "unable to edit coupon");
}