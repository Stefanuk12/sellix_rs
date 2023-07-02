// Dependencies
use serde::{Serialize, Deserialize};

/// Represents a License Product
/// <https://developers.sellix.io/#license>.
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseProduct {
    /// License key purchased by the customer.
    key: String,
    /// UNIQID of the product.
    product_id: String,
    /// Hardware ID of the customer.
    hardware_id: String
}