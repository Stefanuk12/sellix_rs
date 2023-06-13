// Exports
pub mod blacklist;
pub mod subscription;
pub mod license;
pub mod payment;
pub mod events;
pub mod whitelist;
pub mod category;

// Dependencies
use reqwest::{Method, Error};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use strum_macros::FromRepr;
use std::fmt;

/// Includes all of the base sellix http codes the API can respond with.
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr, PartialEq, FromRepr)]
#[repr(u16)]
pub enum SellixHttpCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    NotAcceptable = 406,
    TooManyRequests = 429,
    InternalServerError = 500,
    ServiceUnavailable = 503
}
impl SellixHttpCode {
    // Returns the name of the error code
    fn name(&self) -> &str {
        match self {
            SellixHttpCode::Ok => "Ok",
            SellixHttpCode::BadRequest => "Bad Request",
            SellixHttpCode::Unauthorized => "Unauthorized",
            SellixHttpCode::Forbidden => "Forbidden",
            SellixHttpCode::NotFound => "Not Found",
            SellixHttpCode::NotAcceptable => "Not Acceptable",
            SellixHttpCode::TooManyRequests => "Too Many Requests",
            SellixHttpCode::InternalServerError => "Internal Server Error",
            SellixHttpCode::ServiceUnavailable => "Service Unavailable",
        }
    }

    // Returns a description of the error code
    fn description(&self) -> &str {
        match self {
            SellixHttpCode::Ok => "Request successful",
            SellixHttpCode::BadRequest => "Invalid parameters",
            SellixHttpCode::Unauthorized => "Unable to authenticate",
            SellixHttpCode::Forbidden => "The request is not allowed",
            SellixHttpCode::NotFound => "The specified resource could not be found.",
            SellixHttpCode::NotAcceptable => "You requested a format that isn't json.",
            SellixHttpCode::TooManyRequests => "You have reached the rate limit",
            SellixHttpCode::InternalServerError => "We had a problem with our server. Try again later. These are rare.",
            SellixHttpCode::ServiceUnavailable => "We're temporarily offline for maintenance. Please try again later.",
        }
    }
}
impl std::fmt::Display for SellixHttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.name(), self.description())
    }
}
impl std::error::Error for SellixHttpCode {}

/// A base API response from the Sellix API.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawAPIResponse<T> {
    pub status: SellixHttpCode,
    pub data: Option<T>,
    pub message: Option<String>,
    pub log: Option<String>,
    pub error: Option<String>,
    pub env: String,
}

/// All of the possible requests you can make to the API.
pub enum RequestType {
    BlacklistGet,
    BlacklistList,
    BlacklistCreate,
    BlacklistUpdate,
    BlacklistDestroy,

    WhitelistGet,
    WhitelistList,
    WhitelistCreate,
    WhitelistUpdate,
    WhitelistDestroy,

    CategoryGet,
    CategoryList,
    CategoryCreate,
    CategoryUpdate,
    CategoryDestroy,
}
impl RequestType {
    /// Returns a tuple that describes the method and path corrosponding to the [`RequestType`].
    pub fn request_details(&self) -> (Method, &str) {
        match self {
            RequestType::BlacklistGet => (Method::GET, "/blacklists/{{uniqid}}"),
            RequestType::BlacklistList => (Method::GET, "/blacklists?page={{page}}"),
            RequestType::BlacklistCreate => (Method::POST, "/blacklists"),
            RequestType::BlacklistUpdate => (Method::PUT, "/blacklists/{{uniqid}}"),
            RequestType::BlacklistDestroy => (Method::DELETE, "/blacklists/{{uniqid}}"),

            RequestType::WhitelistGet => (Method::GET, "/whitelists/{{uniqid}}"),
            RequestType::WhitelistList => (Method::GET, "/whitelists?page={{page}}"),
            RequestType::WhitelistCreate => (Method::POST, "/whitelists"),
            RequestType::WhitelistUpdate => (Method::PUT, "/whitelists/{{uniqid}}"),
            RequestType::WhitelistDestroy => (Method::DELETE, "/whitelists/{{uniqid}}"),

            RequestType::CategoryGet => (Method::GET, "/categories/{{uniqid}}"),
            RequestType::CategoryList => (Method::GET, "/categories?page={{page}}"),
            RequestType::CategoryCreate => (Method::POST, "/categories"),
            RequestType::CategoryUpdate => (Method::PUT, "/categories/{{uniqid}}"),
            RequestType::CategoryDestroy => (Method::DELETE, "/categories/{{uniqid}}"),
        }
    }
}

/// All of the supported currencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
pub enum Currencies {
    CAD,
    HKD,
    ISK,
    PHP,
    DKK,
    HUF,
    CZK,
    GBP,
    RON,
    SEK,
    IDR,
    INR,
    BRL,
    RUB,
    HRK,
    JPY,
    THB,
    CHF,
    EUR,
    MYR,
    BGN,
    TRY,
    CNY,
    NOK,
    NZD,
    ZAR,
    USD,
    MXN,
    SGD,
    AUD,
    ILS,
    KRW,
    PLN
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniqidDict {
    pub uniqid: String
}

/// Custom error, since Sellix does not send error codes over HTTP
#[derive(Debug)]
pub struct SellixError {
    pub kind: SellixHttpCode,
    pub message: String,
}
impl fmt::Display for SellixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for SellixError {}
impl From<Error> for SellixError {
    fn from(error: Error) -> Self {
        println!("{}", error);
        SellixError {
            kind: SellixHttpCode::from_repr(error.status().unwrap().as_u16()).unwrap(),
            message: error.to_string(),
        }
    }
}

/// All of the days in the week
#[derive(Debug, Serialize, Deserialize)]
pub enum WeekDays {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun
}

/// All of the months in the year
#[derive(Debug, Serialize, Deserialize)]
pub enum YearMonths {
    Jan,
    Feb,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec
}