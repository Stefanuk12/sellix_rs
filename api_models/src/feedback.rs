// Dependencies
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::{Serialize_repr, Deserialize_repr};
use strum_macros::FromRepr;
use super::RawAPIResponse;
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use std::time::SystemTime;

/// The outcome of an appeal.
/// Used by [`FeedbackRaw`]
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum AppealOutcome {
    NoAppealAvailable,
    Approved,
    Rejected
}

/// The possible feedback scores.
/// Used in [`FeedbackRaw`].
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, FromRepr, strum_macros::EnumString, strum_macros::Display)]
#[repr(u16)]
pub enum FeedbackScore {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5
}

/// Represents the raw API response for a feedback object.
/// <https://developers.sellix.io/#feedback-object>
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackRaw {
    /// ID of the resource.
    id: u64,
    /// Unique ID of the resource,
    /// used as reference across the API.
    uniqid: String,
    /// Unique ID of the product for which this feedback has been posted.
    product_id: String,
    /// Unique ID of the product for which this feedback has been posted.
    invoice_id: String,
    /// If `true`, this feedback has been blocked after an appeal.
    blocked: bool,
    /// If `true`, an appeal has been created for this feedback.
    appealed: bool,
    /// Not always available.
    appeal_outcome: AppealOutcome,
    /// The shop ID to which this feedback belongs.
    shop_id: u64,
    /// Message left by the customer.
    message: String,
    /// Reply left by the merchant.
    reply: String,
    /// Score left by the customer, if 0 no score has been left. From 0 to 5.
    score: FeedbackScore,
    /// Product title for which this feedback has been created.
    product_title: String,
    /// Deprecated
    product_image_name: Option<String>,
    /// Deprecated
    product_image_storage: Option<String>,
    /// New field containing the cloudflare image ID of this product,
    /// replaces image_attachment and image_name.
    /// 
    /// Format https://imagedelivery.net/95QNzrEeP7RU5l5WdbyrKw/
    /// where
    /// 
    /// `variant_name` can be `shopItem`, `avatar`, `icon`, `imageAvatarFeedback`, `public`, `productImageCart`.
    cloudflare_image_id: String,
    /// Contains the full invoice object for this feedback.
    invoice: Value,
    /// Contains the full product object for this feedback.
    product: Value,
    /// Creation data of the product.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    created_at: SystemTime,
    /// Date, available if the product has been edited.
    #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
    updated_at: Option<SystemTime>,
    /// User ID, available if the product has been edited.
    updated_by: Option<u64>
}

/// Raw API response from here.
/// <https://developers.sellix.io/#feedback-get>.
/// Used in [`FeedbackGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackOneRaw {
    pub feedback: FeedbackRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#feedback-get>.
pub type FeedbackGetResponseRaw = RawAPIResponse<FeedbackOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#feedback-list>.
/// Used for [`FeedbackListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackArray {
    pub feedbacks: Vec<FeedbackRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#feedback-list>.
pub type FeedbackListResponseRaw = RawAPIResponse<FeedbackArray>;

/// Payload for replying to feedback.
/// <https://developers.sellix.io/#feedback-reply>.
#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackReplyPayload<'a> {
    pub reply: &'a str
}