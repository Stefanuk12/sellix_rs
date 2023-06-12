// Dependencies
use sellix_rs::ProductQuerystringBuilder;

// Entrypoint
fn main() {
    // Initialise the builder and add all of the stuff
    let builder = ProductQuerystringBuilder::new("test_store", "uniqid")
        .quantity(50)
        .email("example@example.com")
        .add_custom_field("customField", "hello");

    // Log it
    println!("built: {}", builder.build());
}