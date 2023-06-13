// Dependencies
use serde::Serialize;
use serde_json::{Value, json};
use async_trait::async_trait;
use sellix_rs_macros::{WithAPIKey, WithDoRequest};
use crate::models::{blacklist::{BlacklistRaw, BlacklistGetResponseRaw, BlacklistsArray, BlacklistListResponseRaw, BlacklistCreatePayload, BlacklistCreateResponseRaw}, RequestType, RawAPIResponse, SellixHttpCode, UniqidDict, SellixError, whitelist::{WhitelistRaw, WhitelistGetResponseRaw, WhitelistsArray, WhitelistListResponseRaw, WhitelistCreatePayload, WhitelistCreateResponseRaw}, category::{CategoryGetResponseRaw, CategoryRaw, CategoryArray, CategoryCreateResponseRaw, CategoryCreatePayload, CategoryListResponseRaw}};

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
        println!("{}", json);
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
#[derive(WithAPIKey, WithDoRequest)]
pub struct Blacklist {
    pub api_key: String,
    pub merchant: Option<String>
}
impl Blacklist {
    /// Creates a new instance.
    pub fn new(api_key: &str, merchant: Option<&str>) -> Self {
        Self {
            api_key: api_key.to_string(),
            merchant: merchant.and_then(|x| Some(x.to_string()))
        }
    }

    /// Retrieves a Blacklist by ID.
    pub async fn get(&self, uniqid: &str) -> Result<BlacklistRaw, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::BlacklistGet.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<BlacklistGetResponseRaw, BlacklistRaw>(method, &path, None)
            .await
            .and_then(|x| Ok(x.data.unwrap().blacklist))
    }

    /// Returns a list of the Blacklist. The blacklist are sorted by creation date, with the most recently created blacklist being first.
    pub async fn get_list(&self, page: Option<u64>) -> Result<BlacklistsArray, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::BlacklistList.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "page": page
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<BlacklistListResponseRaw, BlacklistsArray>(method, &path, None)
            .await
            .and_then(|x| Ok(x.data.unwrap()))
    }

    /// Creates a Blacklist.
    pub async fn create(&self, payload: BlacklistCreatePayload<'_>) -> Result<UniqidDict, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::BlacklistCreate.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &payload)
            .expect("unable to parse path");

        // Send it
        self.do_request::<BlacklistCreateResponseRaw, BlacklistCreatePayload>(method, &path, Some(payload))
            .await
            .and_then(|x| Ok(x.data.unwrap()))
    }

    /// Edits a Blacklist.
    pub async fn edit(&self, uniqid: &str, payload: BlacklistCreatePayload<'_>) -> Result<bool, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::BlacklistUpdate.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<RawAPIResponse<()>, BlacklistCreatePayload>(method, &path, Some(payload))
            .await
            .and_then(|x| Ok(x.status == SellixHttpCode::Ok))
    }

    /// Deletes a Blacklist.
    pub async fn delete(&self, uniqid: &str) -> Result<bool, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::BlacklistDestroy.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<RawAPIResponse<()>, Value>(method, &path, None)
            .await
            .and_then(|x| Ok(x.status == SellixHttpCode::Ok))
    }
}

/// Allow certain people from accessing your shop.
#[derive(WithAPIKey, WithDoRequest)]
pub struct Whitelist {
    pub api_key: String,
    pub merchant: Option<String>
}
impl Whitelist {
    /// Creates a new instance.
    pub fn new(api_key: &str, merchant: Option<&str>) -> Self {
        Self {
            api_key: api_key.to_string(),
            merchant: merchant.and_then(|x| Some(x.to_string()))
        }
    }

    /// Retrieves a Whitelist by ID.
    pub async fn get(&self, uniqid: &str) -> Result<WhitelistRaw, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::WhitelistGet.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<WhitelistGetResponseRaw, WhitelistRaw>(method, &path, None)
            .await
            .and_then(|x| Ok(x.data.unwrap().whitelist))
    }

    /// Returns a list of the Whitelist. The whitelist are sorted by creation date, with the most recently created whitelist being first.
    pub async fn get_list(&self, page: Option<u64>) -> Result<WhitelistsArray, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::WhitelistList.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "page": page
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<WhitelistListResponseRaw, WhitelistsArray>(method, &path, None)
            .await
            .and_then(|x| Ok(x.data.unwrap()))
    }

    /// Creates a Whitelist.
    pub async fn create(&self, payload: WhitelistCreatePayload<'_>) -> Result<UniqidDict, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::WhitelistCreate.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &payload)
            .expect("unable to parse path");

        // Send it
        self.do_request::<WhitelistCreateResponseRaw, WhitelistCreatePayload>(method, &path, Some(payload))
            .await
            .and_then(|x| Ok(x.data.unwrap()))
    }

    /// Edits a Whitelist.
    pub async fn edit(&self, uniqid: &str, payload: WhitelistCreatePayload<'_>) -> Result<bool, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::WhitelistUpdate.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<RawAPIResponse<()>, WhitelistCreatePayload>(method, &path, Some(payload))
            .await
            .and_then(|x| Ok(x.status == SellixHttpCode::Ok))
    }

    /// Deletes a Whitelist.
    pub async fn delete(&self, uniqid: &str) -> Result<bool, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::WhitelistDestroy.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<RawAPIResponse<()>, Value>(method, &path, None)
            .await
            .and_then(|x| Ok(x.status == SellixHttpCode::Ok))
    }
}

/// Categories
#[derive(WithAPIKey, WithDoRequest)]
pub struct Category {
    pub api_key: String,
    pub merchant: Option<String>
}
impl Category {
    /// Creates a new instance.
    pub fn new(api_key: &str, merchant: Option<&str>) -> Self {
        Self {
            api_key: api_key.to_string(),
            merchant: merchant.and_then(|x| Some(x.to_string()))
        }
    }

    /// Retrieves a Category by ID.
    pub async fn get(&self, uniqid: &str) -> Result<CategoryRaw, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::CategoryGet.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<CategoryGetResponseRaw, CategoryRaw>(method, &path, None)
            .await
            .and_then(|x| Ok(x.data.unwrap().category))
    }

    /// Returns a list of the Category. The whitelist are sorted by creation date, with the most recently created whitelist being first.
    pub async fn get_list(&self, page: Option<u64>) -> Result<CategoryArray, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::CategoryList.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "page": page
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<CategoryListResponseRaw, CategoryArray>(method, &path, None)
            .await
            .and_then(|x| Ok(x.data.unwrap()))
    }

    /// Creates a Category.
    pub async fn create(&self, payload: CategoryCreatePayload<'_>) -> Result<UniqidDict, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::CategoryCreate.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &payload)
            .expect("unable to parse path");

        // Send it
        self.do_request::<CategoryCreateResponseRaw, CategoryCreatePayload>(method, &path, Some(payload))
            .await
            .and_then(|x| Ok(x.data.unwrap()))
    }

    /// Edits a Category.
    pub async fn edit(&self, uniqid: &str, payload: CategoryCreatePayload<'_>) -> Result<bool, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::CategoryUpdate.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<RawAPIResponse<UniqidDict>, CategoryCreatePayload>(method, &path, Some(payload))
            .await
            .and_then(|x| Ok(x.status == SellixHttpCode::Ok))
    }

    /// Deletes a Category.
    pub async fn delete(&self, uniqid: &str) -> Result<bool, SellixError> {
        // Used to build the url
        let (method, path_builder) = RequestType::CategoryDestroy.request_details();
        let path = handlebars::Handlebars::new()
            .render_template(path_builder, &json!({
                "uniqid": uniqid
            }))
            .expect("unable to parse path");

        // Send it
        self.do_request::<RawAPIResponse<()>, Value>(method, &path, None)
            .await
            .and_then(|x| Ok(x.status == SellixHttpCode::Ok))
    }
}