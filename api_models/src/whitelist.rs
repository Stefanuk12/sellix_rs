// Dependencies
use serde::{Serialize, Deserialize};
use super::{RawAPIResponse, UniqidDict};
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use std::time::SystemTime;

/// The types of whitelist.
/// Used by [`WhitelistRaw`].
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum WhitelistTypes {
    Email,
    IP,
    Country,
    ISP,
    ASN,
    Host
}

/// Represents the raw API response for a whitelist object.
/// <https://developers.sellix.io/#whitelists>.
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistRaw {
    /// ID of the resource
    pub id: u64,
    /// Unique ID of the resource, used as reference across the API.
    pub uniqid: String,
    /// The shop ID to which this whitelist belongs.
    pub shop_id: u64,
    /// The type of data of this whitelist.
    pub r#type: WhitelistTypes,
    /// The value of the whitelist.
    pub data: String,
    /// Additional note provided on whitelist creation.
    pub note: String,
    /// Creation date of the whitelist.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// Date, available if the whitelist has been edited.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// User ID, available if the whitelist has been edited.
    pub updated_by: u64,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#whitelist-get>.
/// Used in [`WhitelistGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistOneRaw {
    pub whitelist: WhitelistRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#whitelist-get>.
pub type WhitelistGetResponseRaw = RawAPIResponse<WhitelistOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#whitelist-list>.
/// Used for [`WhitelistListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistArray {
    pub whitelists: Vec<WhitelistRaw>
}
pub type WhitelistListResponseRaw = RawAPIResponse<WhitelistArray>;

/// Represents the payload for creating (or updating) a whitelist.
/// <https://developers.sellix.io/#whitelist-create>.
/// <https://developers.sellix.io/#whitelist-update>.
#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistCreatePayload<'a> {
    /// The type of data of this whitelist.
    pub r#type: WhitelistTypes,
    /// Blocked data. Either country code, email or IP address
    pub data: &'a str,
    /// Internal note for the reasoning of the whitelist
    pub note: Option<&'a str>
}

/// Represents the response after creating a whitelist.
/// <https://developers.sellix.io/#whitelist-create>.
/// Used for [`WhitelistCreateResponseRaw`]
/// Represents the response after creating a whitelist.
/// <https://developers.sellix.io/#whitelist-create>.
pub type WhitelistCreateResponseRaw = RawAPIResponse<UniqidDict>;