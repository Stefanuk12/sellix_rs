// Dependencies
use serde::{Serialize, Deserialize};
use super::{RawAPIResponse, UniqidDict};
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use std::time::SystemTime;

/// The scope of a blacklist.
/// Used by [`BlacklistRaw`].
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum BlacklistScope {
    Private,
    Shared
}

/// The types of blacklist.
/// Used by [`BlacklistRaw`].
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum BlacklistTypes {
    Email,
    IP,
    Country,
    ISP,
    ASN,
    Host
}

/// Represents the raw API response for a blacklist object.
/// <https://developers.sellix.io/#blacklists>.
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct BlacklistRaw {
    /// ID of the resource
    pub id: u64,
    /// Unique ID of the resource, used as reference across the API.
    pub uniqid: String,
    /// Whether it is a `PRIVATE` or `SHARED` blacklist.
    /// `SHARED` blacklists are created by Sellix's Fraud Shield to be used across Business and Enterprise merchants.
    pub scope: BlacklistScope,
    /// The shop ID to which this blacklist belongs.
    pub shop_id: u64,
    /// The type of data of this blacklist.
    pub r#type: BlacklistTypes,
    /// The value of the blacklist.
    pub data: String,
    /// Additional note provided on blacklist creation.
    pub note: String,
    /// Creation date of the blacklist.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// Date, available if the blacklist has been edited.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// User ID, available if the blacklist has been edited.
    pub updated_by: u64,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#blacklist-get>.
/// Used in [`BlacklistGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct BlacklistOneRaw {
    pub blacklist: BlacklistRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#blacklist-get>.
pub type BlacklistGetResponseRaw = RawAPIResponse<BlacklistOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#blacklist-list>.
/// Used for [`BlacklistListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct BlacklistArray {
    pub blacklists: Vec<BlacklistRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#blacklist-list>.
pub type BlacklistListResponseRaw = RawAPIResponse<BlacklistArray>;

/// Represents the payload for creating (or updating) a blacklist.
/// <https://developers.sellix.io/#blacklist-create>.
/// <https://developers.sellix.io/#blacklist-update>.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlacklistCreatePayload<'a> {
    /// The type of data of this blacklist.
    pub r#type: BlacklistTypes,
    /// Blocked data. Either country code, email or IP address
    pub data: &'a str,
    /// Internal note for the reasoning of the blacklist
    pub note: Option<&'a str>
}

/// Represents the response after creating a blacklist.
/// <https://developers.sellix.io/#blacklist-create>.
pub type BlacklistCreateResponseRaw = RawAPIResponse<UniqidDict>;