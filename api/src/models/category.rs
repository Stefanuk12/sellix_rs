// Dependencies
use std::time::SystemTime;
use serde_json::Value;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;
use serde::{Deserialize, Serialize};

use super::{payment::{PaymentGateway, DiscountType}, subscription::RecurringBillingIntervals, Currencies, RawAPIResponse, UniqidDict};

/// Used in [`CategoryRaw`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    pub total: u64,
    pub positive: u64,
    pub neutral: u64,
    pub negative: u64,
}

/// Used in [`CategoryRaw`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomField {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub regex: Option<String>,
    pub placeholder: Option<String>,
    pub default: Option<String>,
    pub required: bool,
}

/// Used in [`CategoryRaw`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeDiscount {
    #[serde(rename = "type")]
    pub type_field: DiscountType,
    pub value: u64,
    pub quantity: u64,
}

/// Used in [`CategoryRaw`].
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductsBound {
    pub id: u64,
    pub uniqid: String,
    pub shop_id: u64,
    pub type_field: String,
    pub subtype: Option<String>,
    pub title: String,
    pub currency: Currencies,
    pub price: f64,
    pub price_display: f64,
    pub description: String,
    pub image_attachment: Option<String>,
    pub file_attachment: Option<String>,
    pub volume_discounts: Vec<VolumeDiscount>,
    pub recurring_interval: RecurringBillingIntervals,
    pub recurring_interval_count: u64,
    pub trial_period: u64,
    pub paypal_product_id: Option<String>,
    pub paypal_plan_id: Option<String>,
    pub stripe_price_id: String,
    pub quantity_min: i32,
    pub quantity_max: i32,
    pub quantity_warning: u64,
    pub gateways: Vec<PaymentGateway>,
    pub custom_fields: Vec<CustomField>,
    pub crypto_confirmations_needed: u64,
    pub max_risk_level: u64,
    pub block_vpn_proxies: bool,
    pub delivery_text: String,
    pub service_text: String,
    pub stock_delimiter: String,
    pub stock: i32,
    pub dynamic_webhook: Option<String>,
    pub sort_priority: u64,
    pub unlisted: bool,
    pub on_hold: bool,
    pub terms_of_service: Option<String>,
    pub warranty: u64,
    pub warranty_text: String,
    pub private: bool,
    pub name: String,
    pub image_name: Option<String>,
    pub image_storage: Option<String>,
    pub cloudflare_image_id: String,
    pub serials: Vec<String>,
    pub webhooks: Vec<String>,
    pub feedback: Feedback,
    pub theme: String,
    pub dark_mode: u64,
    pub average_score: f64,
    pub sold_count: u64,
    pub lex_payment_methods: Vec<String>,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    pub updated_by: u64,
}

/// Used in [`CategoryRaw`].
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

/// Represents the raw API response for a category object.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryRaw {
    // ID of the resource.
    pub id: u64,
    /// Unique ID of the resource, used as reference across the API.
    pub uniqid: String,
    /// The shop ID to which this category belongs..
    pub shop_id: u64,
    /// The shop ID to which this category belongs.
    pub title: String,
    /// The shop ID to which this category belongs.
    pub unlisted: bool,
    /// Sort order of this category.
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
    /// Creation date of the category.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub created_at: SystemTime,
    /// Creation date of the category.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub updated_at: SystemTime,
    /// User ID, available if the category has been edited.
    pub updated_by: u64,
}

/// Raw API response from here.
/// <https://developers.sellix.io/#category-get>.
/// Used in [`CategoryGetResponseRaw`]
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryOneRaw {
    pub category: CategoryRaw
}

/// Raw API response from here.
/// <https://developers.sellix.io/#category-get>.
pub type CategoryGetResponseRaw = RawAPIResponse<CategoryOneRaw>;

/// Raw API response from here.
/// <https://developers.sellix.io/#category-list>.
/// Used for [`BlacklistListResponseRaw`].
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryArray {
    pub categories: Vec<CategoryRaw>
}
/// Raw API response from here.
/// <https://developers.sellix.io/#category-list>.
pub type CategoryListResponseRaw = RawAPIResponse<CategoryArray>;

/// Represents the response after creating a category.
/// <https://developers.sellix.io/#category-create>.
pub type CategoryCreateResponseRaw = RawAPIResponse<UniqidDict>;

/// Represents the payload for creating (or updating) a category.
/// <https://developers.sellix.io/#category-create>.
/// <https://developers.sellix.io/#category-update>.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCreatePayload<'a> {
    pub title: &'a str,
    pub unlisted: Option<bool>,
    pub products_bound: Option<Vec<ProductsBound>>,
    pub groups_array: Option<Vec<GroupsBound>>,
    pub sort_priority: Option<u64>
}