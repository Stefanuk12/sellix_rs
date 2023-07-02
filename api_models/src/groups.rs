// Dependencies
use std::time::SystemTime;
use serde_json::Value;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;
use serde::{Deserialize, Serialize};

use super::{RawAPIResponse, UniqidDict, product::ProductsBound};

/// Used in [`GroupRaw`].
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupsBound {
    pub uniqid: String,
    pub title: String,
    pub image_attachment: Value,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
}

/// Represents the raw API response for a group object.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupRaw {
    // ID of the resource.
    pub id: u64,
    /// Unique ID of the resource, used as reference across the API.
    pub uniqid: String,
    /// The shop ID to which this group belongs..
    pub shop_id: u64,
    /// The shop ID to which this group belongs.
    pub title: String,
    /// The shop ID to which this group belongs.
    pub unlisted: bool,
    /// Sort order of this group.
    pub sort_priority: u64,
    /// Array of products.
    /// Please note that the product object contains limited fields, 
    /// to get the full product object please use the Products API endpoint.
    pub products_bound: Vec<ProductsBound>,
    /// How many products are present in the products_bound array.
    pub products_count: u64,
    /// Array of groups.
    pub groups_bound: Vec<GroupsBound>,
    /// How many groups are present in the groups_bound array.
    pub groups_count: u64,
    /// Creation date of the group.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// Creation date of the group.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// User ID, available if the group has been edited.
    pub updated_by: u64,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#group-get>.
/// Used in [`GroupGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupOneRaw {
    pub group: GroupRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#group-get>.
pub type GroupGetResponseRaw = RawAPIResponse<GroupOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#group-list>.
/// Used for [`GroupListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupArray {
    pub groups: Vec<GroupRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#group-list>.
pub type GroupListResponseRaw = RawAPIResponse<GroupArray>;

/// Represents the response after creating a group.
/// <https://developers.sellix.io/#group-create>.
pub type GroupCreateResponseRaw = RawAPIResponse<UniqidDict>;

/// Represents the payload for creating (or updating) a group.
/// <https://developers.sellix.io/#group-create>.
/// <https://developers.sellix.io/#group-update>.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupCreatePayload<'a> {
    pub title: &'a str,
    pub unlisted: Option<bool>,
    pub products_bound: Option<Vec<ProductsBound>>,
    pub groups_array: Option<Vec<GroupsBound>>,
    pub sort_priority: Option<u64>
}