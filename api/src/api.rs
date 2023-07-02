// Dependencies
use sellix_api_models::{SellixError, SellixHttpCode, RawAPIResponse, feedback::FeedbackReplyPayload, RequestType};
use serde::Serialize;
use serde_json::{Value, json};
use async_trait::async_trait;
use sellix_macros::{WithAPIKey, WithDoRequest, DefaultAPI};

// Constants
const API_BASE: &str = "https://dev.sellix.io/v1";

/// Ensures that the API key exists within trait.
pub trait WithAPIKey {
    fn api_key(&self) -> String;
    fn merchant(&self) -> Option<String>;
}

/// Used within API models to add support for sending HTTP requests to the API.
#[async_trait]
pub trait DoRequest: WithAPIKey {
    /// Performs an API request.
    /// `T` must be a [`RawAPIResponse`].
    async fn do_request<T: for<'de> serde::Deserialize<'de>, B: Serialize + std::marker::Send>(&self, method: reqwest::Method, path: &str, body: Option<B>) -> Result<T, SellixError> {
        // Create the client
        let client = reqwest::Client::new();
        
        // Create the request, set url, body, headers...
        let url = reqwest::Url::parse(&(API_BASE.to_owned() + path)).unwrap();
        let request = client.request(method, url)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .header("X-Sellix-Merchant", self.merchant().unwrap_or(String::from("")))
            .json(&body);

        // Grab the response
        let response = request.send().await?;
        let json = response.json::<Value>().await?;

        // Check the status
        let status = json["status"].as_u64().unwrap();
        if status == 200 {
            let json_resp: T = serde_json::from_value(json).unwrap();
            return Ok(json_resp);
        }
        
        // Uh oh error
        let status_code = SellixHttpCode::from_repr(status.try_into().unwrap()).unwrap();
        Err(SellixError {
            kind: status_code.clone(),
            message: status_code.to_string()
        })
    }
}

/// Disallow certain people from accessing your shop.
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
pub struct Blacklist {
    pub api_key: String,
    pub merchant: Option<String>
}

/// Allow certain people from accessing your shop.
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
pub struct Whitelist {
    pub api_key: String,
    pub merchant: Option<String>
}

/// Categories
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
pub struct Category {
    pub api_key: String,
    pub merchant: Option<String>
}
/// Provide a discount to your products.
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
pub struct Coupon {
    pub api_key: String,
    pub merchant: Option<String>
}

/// Manage feedback.
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
#[api_methods(create=false,edit=false,delete=false)]
pub struct Feedback {
    pub api_key: String,
    pub merchant: Option<String>
}
impl Feedback {
    /// Replies to a Feedback.
    pub async fn reply(&self, uniqid: &str, reply: &str) -> Result<bool, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::CouponCreate.request_details();
        let payload = FeedbackReplyPayload {
            reply
        };
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<RawAPIResponse<()>, FeedbackReplyPayload>(method, &path, Some(payload))
            .await
            .and_then(|x| Ok(x.status == SellixHttpCode::Ok))
    }
}

/// Customers
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
pub struct Customer {
    pub api_key: String,
    pub merchant: Option<String>
}

/// Queries
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
pub struct Query {
    pub api_key: String,
    pub merchant: Option<String>
}

/// Subscriptions
#[derive(WithAPIKey, WithDoRequest, DefaultAPI)]
#[api_methods(edit=false)]
pub struct Subscription {
    pub api_key: String,
    pub merchant: Option<String>
}