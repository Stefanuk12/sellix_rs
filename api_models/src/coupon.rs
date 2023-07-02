// Dependencies
use serde::{Serialize, Deserialize};
use super::payment::DiscountType;
use super::{Currencies, RawAPIResponse, UniqidDict};
use serde_with::TimestampSeconds;
use serde_with::formats::Flexible;
use std::time::SystemTime;

/// Possible coupon types.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum CouponType {
    Product,
    Subscription
}

/// Describes how many times the coupon can be used.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all="UPPERCASE")]
#[serde(rename_all="UPPERCASE")]
pub enum CouponUseType {
    Limited,
    Any
}

/// Represents the raw API response for a coupon object.
/// <https://developers.sellix.io/#coupon-object>.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CouponRaw {
    /// ID of the resource.
    pub id: u64,
    /// Unique ID of the resource, used as reference across the API.
    pub uniqid: String,
    /// The shop ID to which this coupon belongs.
    pub shop_id: u64,
    /// Coupon type, whether it's for a product or subscription.
    /// `SUBSCRIPTION` coupons are only for Sellix-own subscriptions, 
    /// this field will always return `PRODUCT`.
    #[serde(rename = "type")]
    pub type_field: CouponType,
    /// Coupon code to be used during the checkout phase.
    pub code: String,
    /// If `LIMITED`, an array of products must be specified as this coupon will be used only with them.
    pub use_type: CouponUseType,
    /// Discount value for this coupon.
    pub discount: u64,
    /// Deprecated
    pub currency: Option<Currencies>,
    /// How many times this coupon has been used.
    pub used: u64,
    /// Whether or not this coupon is valid if a volume discount is applied.
    pub disabled_with_volume_discounts: bool,
    /// Whether or not this coupon should be applied for each product `SUBSCRIPTION` renewal.
    pub all_recurring_bill_invoices: bool,
    /// Maximum usage for this coupon.
    pub max_uses: i64,
    /// If set, the coupon will expire at that date.
    #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
    pub expire_at: Option<SystemTime>,
    /// Array of product uniqids.
    /// Differs from the categories API as this endpoint does not need specific details about a product.
    /// Use the products API to get details about a single product.
    pub products_bound: Option<Vec<String>>,
    /// How many products are present in the products_bound array.
    pub products_count: u64,
    /// Creation date of the category.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// Date, available if the category has been edited.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// User ID, available if the category has been edited.
    pub updated_by: u64,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#coupon-get>.
/// Used in [`CouponGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponOneRaw {
    pub coupon: CouponRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#coupon-get>.
pub type CouponGetResponseRaw = RawAPIResponse<CouponOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#coupon-list>.
/// Used for [`CouponListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponArray {
    pub coupons: Vec<CouponRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#coupon-list>.
pub type CouponListResponseRaw = RawAPIResponse<CouponArray>;

/// Represents the payload for creating (or updating) a coupon.
/// <https://developers.sellix.io/#coupon-create>.
/// <https://developers.sellix.io/#coupon-update>.
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponCreatePayload<'a> {
    /// Code of the Coupon.
    pub code: &'a str,
    /// Percentage amount of the discount.
    pub discount_value: u64,
    /// How many times can the coupon be used, defaulted to -1.
    pub max_uses: Option<i32>,
    /// Array of products uniqids that the category will contain.
    pub products_bound: Option<Vec<&'a str>>,
    pub discount_type: Option<DiscountType>,
    /// Deprecated.
    pub discount_order_type: Option<&'a str>,
    /// Whether or not this coupon is valid if a volume discount is applied.
    pub disabled_with_volume_discounts: Option<bool>,
    /// Whether or not this coupon should be applied for each product `SUBSCRIPTION` renewal.
    pub all_recurring_bill_invoices: Option<bool>,
    #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
    pub expire_at: Option<SystemTime>
}

/// Represents the response after creating a coupon.
/// <https://developers.sellix.io/#coupon-create>.
pub type CouponCreateResponseRaw = RawAPIResponse<UniqidDict>;