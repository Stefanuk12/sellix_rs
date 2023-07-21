// Exports
pub mod blacklist;
pub mod subscription;
pub mod license;
pub mod payment;
pub mod events;
pub mod whitelist;
pub mod category;
pub mod coupon;
pub mod feedback;
pub mod order;
pub mod product;
pub mod invoice;
pub mod dispute;
pub mod group;
pub mod customer;
pub mod query;
pub mod webhook;

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

    CouponGet,
    CouponList,
    CouponCreate,
    CouponUpdate,
    CouponDestroy,

    FeedbackGet,
    FeedbackList,
    FeedbackReply,

    OrderGet,
    OrderList,
    OrderCreate,
    OrderUpdate,
    OrderDestroy,

    GroupGet,
    GroupList,
    GroupCreate,
    GroupUpdate,
    GroupDestroy,

    CustomerGet,
    CustomerList,
    CustomerCreate,
    CustomerUpdate,
    CustomerDestroy,

    QueryGet,
    QueryList,
    QueryCreate,
    QueryUpdate,
    QueryDestroy,

    SubscriptionGet,
    SubscriptionList,
    SubscriptionCreate,
    SubscriptionDestroy,
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

            RequestType::CouponGet => (Method::GET, "/coupons/{{uniqid}}"),
            RequestType::CouponList => (Method::GET, "/coupons?page={{page}}"),
            RequestType::CouponCreate => (Method::POST, "/coupons"),
            RequestType::CouponUpdate => (Method::PUT, "/coupons/{{uniqid}}"),
            RequestType::CouponDestroy => (Method::DELETE, "/coupons/{{uniqid}}"),

            RequestType::FeedbackGet => (Method::GET, "/feedback/{{uniqid}}"),
            RequestType::FeedbackList => (Method::GET, "/feedback?page={{page}}"),
            RequestType::FeedbackReply => (Method::POST, "/feedback/reply/{{uniqid}}"),

            RequestType::OrderGet => (Method::GET, "/orders/{{uniqid}}"),
            RequestType::OrderList => (Method::GET, "/orders?page={{page}}"),
            RequestType::OrderCreate => (Method::POST, "/orders"),
            RequestType::OrderUpdate => (Method::PUT, "/orders/{{uniqid}}"),
            RequestType::OrderDestroy => (Method::DELETE, "/orders/{{uniqid}}"),

            RequestType::GroupGet => (Method::GET, "/groups/{{uniqid}}"),
            RequestType::GroupList => (Method::GET, "/groups?page={{page}}"),
            RequestType::GroupCreate => (Method::POST, "/groups"),
            RequestType::GroupUpdate => (Method::PUT, "/groups/{{uniqid}}"),
            RequestType::GroupDestroy => (Method::DELETE, "/groups/{{uniqid}}"),
    
            RequestType::CustomerGet => (Method::GET, "/customers/{{uniqid}}"),
            RequestType::CustomerList => (Method::GET, "/customers?page={{page}}"),
            RequestType::CustomerCreate => (Method::POST, "/customers"),
            RequestType::CustomerUpdate => (Method::PUT, "/customers/{{uniqid}}"),
            RequestType::CustomerDestroy => (Method::DELETE, "/customers/{{uniqid}}"),

            RequestType::QueryGet => (Method::GET, "/queries/{{uniqid}}"),
            RequestType::QueryList => (Method::GET, "/queries?page={{page}}"),
            RequestType::QueryCreate => (Method::POST, "/queries"),
            RequestType::QueryUpdate => (Method::PUT, "/queries/{{uniqid}}"),
            RequestType::QueryDestroy => (Method::DELETE, "/queries/{{uniqid}}"),

            RequestType::SubscriptionGet => (Method::GET, "/subscriptions/{{uniqid}}"),
            RequestType::SubscriptionList => (Method::GET, "/subscriptions?page={{page}}"),
            RequestType::SubscriptionCreate => (Method::POST, "/subscriptions"),
            RequestType::SubscriptionDestroy => (Method::DELETE, "/subscriptions/{{uniqid}}"),
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
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
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
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
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