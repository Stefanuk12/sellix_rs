// Dependencies
use std::time::SystemTime;
use serde_json::Value;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;
use serde::{Deserialize, Serialize};

use super::{RawAPIResponse, UniqidDict, groups::GroupsBound, product::ProductsBound};

/// The status of a Query.
/// Used in [`QueryRaw`]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum QueryStatus {
    Pending,
    Closed,
    ShopReply,
    CustomerReply
}

/// Represents the raw API response for a query message object.
/// Used in [`QueryRaw`]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryMessage {
    pub role: String,
    pub message: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
}

/// Represents the raw API response for a query object.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryRaw {
    /// ID of the resource.
    pub id: u64,
    /// Unique ID of the resource, used as reference across the API.
    pub uniqid: String,
    /// The shop ID to which this query belongs..
    pub shop_id: u64,
    /// Unique ID of the invoice this query is linked to, if specified by the customer.
    pub invoice_id: Value,
    pub customer_email: String,
    pub title: String,
    pub status: String,
    pub messages: Vec<QueryMessage>,
    pub day_value: i64,
    pub day: String,
    pub month: String,
    pub year: i64,
    /// Creation date of the query.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// Creation date of the query.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// User ID, available if the query has been edited.
    pub updated_by: u64,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#query-get>.
/// Used in [`QueryGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOneRaw {
    pub query: QueryRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#query-get>.
pub type QueryGetResponseRaw = RawAPIResponse<QueryOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#query-list>.
/// Used for [`QueryListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryArray {
    pub categories: Vec<QueryRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#query-list>.
pub type QueryListResponseRaw = RawAPIResponse<QueryArray>;

/// Represents the response after creating a query.
/// <https://developers.sellix.io/#query-create>.
pub type QueryCreateResponseRaw = RawAPIResponse<UniqidDict>;

/// Represents the payload for creating (or updating) a query.
/// <https://developers.sellix.io/#query-create>.
/// <https://developers.sellix.io/#query-update>.
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCreatePayload<'a> {
    pub title: &'a str,
    pub unlisted: Option<bool>,
    pub products_bound: Option<Vec<ProductsBound>>,
    pub groups_array: Option<Vec<GroupsBound>>,
    pub sort_priority: Option<u64>
}