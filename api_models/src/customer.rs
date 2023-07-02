// Dependencies
use serde::{Deserialize, Serialize};

use super::{RawAPIResponse, UniqidDict};

/// Represents the raw API response for a customer object.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerRaw {
    /// Customer ID
    pub id: String,
    /// ID of the store this customer is connected to
    pub shop_id: u64,
    /// Customer name
    pub name: String,
    /// Customer surname
    pub surname: String,
    /// Customer phone
    pub phone: String,
    /// Customer phone country code
    pub phone_country_code: String,
    /// Customer country code
    pub country_code: String,
    /// Customer street address
    pub address: String,
    /// Customer street address additional info
    pub additional_address_info: String,
    /// Customer city
    pub city: String,
    /// Customer postal code
    pub postal_code: String,
    /// Customer state
    pub state: String,
    /// Customer email
    pub email: String,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#customer-get>.
/// Used in [`CustomerGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerOneRaw {
    pub customer: CustomerRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#customer-get>.
pub type CustomerGetResponseRaw = RawAPIResponse<CustomerOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#customer-list>.
/// Used for [`CustomerListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerArray {
    pub customers: Vec<CustomerRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#customer-list>.
pub type CustomerListResponseRaw = RawAPIResponse<CustomerArray>;

/// Represents the response after creating a customer.
/// <https://developers.sellix.io/#customer-create>.
pub type CustomerCreateResponseRaw = RawAPIResponse<UniqidDict>;

/// Represents the payload for creating (or updating) a customer.
/// <https://developers.sellix.io/#customer-create>.
/// <https://developers.sellix.io/#customer-update>.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerCreatePayload<'a> {
    /// Customer name
    pub name: &'a str,
    /// Customer surname
    pub surname: &'a str,
    /// Customer phone
    pub phone: Option<&'a str>,
    /// Customer phone country code
    pub phone_country_code: Option<&'a str>,
    /// Customer country code
    pub country_code: Option<&'a str>,
    /// Customer street address
    pub address: Option<&'a str>,
    /// Customer street address additional info
    pub additional_address_info: Option<&'a str>,
    /// Customer city
    pub city: Option<&'a str>,
    /// Customer postal code
    pub postal_code: Option<&'a str>,
    /// Customer state
    pub state: Option<&'a str>,
    /// Customer email
    pub email: &'a str,
}