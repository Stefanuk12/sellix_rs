// Dependencies
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};
use darling::FromDeriveInput;

/// Ensures an API model has an api_key and merchant field.
/// Used for do_request.
#[proc_macro_derive(WithAPIKey)]
pub fn with_api_key_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct we're deriving the trait for.
    let name = input.ident;

    // Generate the implementation of the trait.
    let expanded = quote! {  
        impl WithAPIKey for #name {
            fn api_key(&self) -> String {
                self.api_key.clone()
            }
            fn merchant(&self) -> Option<String> {
                self.merchant.clone()
            }
        }
    };

    // Return the generated code as tokens.
    TokenStream::from(expanded)
}

/// Implements `DoRequest` for a struct.
/// 
/// `impl DoRequest for #name {}`
#[proc_macro_derive(WithDoRequest)]
pub fn with_do_request_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl DoRequest for #name {}
    };

    gen.into()
}

/// The options for [`DefaultAPI`].
#[derive(FromDeriveInput, Default)]
#[darling(forward_attrs, attributes(api_methods))]
struct DefaultAPIOpts {
    new: Option<bool>,
    get: Option<bool>,
    list: Option<bool>,
    create: Option<bool>,
    edit: Option<bool>,
    delete: Option<bool>,

    #[darling(default)]
    _non_exhaustive: (),
}

/// Adds default API methods.
#[proc_macro_derive(DefaultAPI, attributes(api_methods))]
pub fn default_api_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input);
    let opts = match DefaultAPIOpts::from_derive_input(&input) {
        Ok(options) => options,
        Err(err) => return err.write_errors().into(),
    };
    let DeriveInput { ident, .. } = input;
    let name = format_ident!("{}", ident.to_string().to_lowercase());
    
    // All of the structs/enum names that could be used...
    let x_raw = format_ident!("{}Raw", ident);
    let x_get = format_ident!("{}Get", ident);
    let x_array = format_ident!("{}Array", ident);
    let x_list = format_ident!("{}List", ident);
    let x_create = format_ident!("{}Create", ident);
    let x_update = format_ident!("{}Update", ident);
    let x_destroy = format_ident!("{}Destroy", ident);
    let x_get_response = format_ident!("{}GetResponseRaw", ident);
    let x_list_response = format_ident!("{}ListResponseRaw", ident);
    let x_create_response_raw = format_ident!("{}CreateResponseRaw", ident);
    let x_create_payload = format_ident!("{}CreatePayload", ident);

    // Each of the default methods...
    let empty = quote! {};
    let client = quote! {
        /// Creates a new instance client.
        pub fn client(api_key: &str, merchant: Option<&str>) -> Self {
            Self {
                api_key: api_key.to_string(),
                merchant: merchant.and_then(|x| Some(x.to_string()))
            }
        }
    };
    let client = if opts.new.unwrap_or(true) {client} else {empty.clone()};
    let get = quote! {
        /// Retrieves a instance of this class by its uniqid.
        pub async fn get(&self, uniqid: &str) -> Result<sellix_api_models::#name::#x_raw, sellix_api_models::SellixError> {
            // Used to build the url
            let (method, path_builder) = sellix_api_models::RequestType::#x_get.request_details();
            let path = handlebars::Handlebars::new()
                .render_template(path_builder, &json!({
                    "uniqid": uniqid
                }))
                .expect("unable to parse path");

            // Send it
            self.do_request::<sellix_api_models::#name::#x_get_response, sellix_api_models::#name::#x_raw>(method, &path, None)
                .await
                .and_then(|x| Ok(x.data.unwrap().#name))
        }
    };
    let get = if opts.get.unwrap_or(true) {get} else {empty.clone()};
    let list = quote! {
        /// Returns a list of instances.
        /// Sorted by creation date.
        pub async fn get_list(&self, page: Option<u64>) -> Result<sellix_api_models::#name::#x_array, sellix_api_models::SellixError> {
            // Used to build the url
            let (method, path_builder) = sellix_api_models::RequestType::#x_list.request_details();
            let path = handlebars::Handlebars::new()
                .render_template(path_builder, &json!({
                    "page": page
                }))
                .expect("unable to parse path");

            // Send it
            self.do_request::<sellix_api_models::#name::#x_list_response, sellix_api_models::#name::#x_array>(method, &path, None)
                .await
                .and_then(|x| Ok(x.data.unwrap()))
        }
    };
    let list = if opts.list.unwrap_or(true) {list} else {empty.clone()};
    let create = quote! {
        /// Creates a instance to the API.
        pub async fn create(&self, payload: sellix_api_models::#name::#x_create_payload<'_>) -> Result<sellix_api_models::UniqidDict, sellix_api_models::SellixError> {
            // Used to build the url
            let (method, path_builder) = sellix_api_models::RequestType::#x_create.request_details();
            let path = handlebars::Handlebars::new()
                .render_template(path_builder, &payload)
                .expect("unable to parse path");

            // Send it
            self.do_request::<sellix_api_models::#name::#x_create_response_raw, sellix_api_models::#name::#x_create_payload>(method, &path, Some(payload))
                .await
                .and_then(|x| Ok(x.data.unwrap()))
        }
    };
    let create = if opts.create.unwrap_or(true) {create} else {empty.clone()};
    let edit = quote! {
        /// Edits an instance via API.
        pub async fn edit(&self, uniqid: &str, payload: sellix_api_models::#name::#x_create_payload<'_>) -> Result<bool, sellix_api_models::SellixError> {
            // Used to build the url
            let (method, path_builder) = sellix_api_models::RequestType::#x_update.request_details();
            let path = handlebars::Handlebars::new()
                .render_template(path_builder, &json!({
                    "uniqid": uniqid
                }))
                .expect("unable to parse path");

            // Send it
            self.do_request::<sellix_api_models::RawAPIResponse<()>, sellix_api_models::#name::#x_create_payload>(method, &path, Some(payload))
                .await
                .and_then(|x| Ok(x.status == sellix_api_models::SellixHttpCode::Ok))
        }
    };
    let edit = if opts.edit.unwrap_or(true) {edit} else {empty.clone()};
    let delete = quote! {
        /// Deletes an instance via API.
        pub async fn delete(&self, uniqid: &str) -> Result<bool, sellix_api_models::SellixError> {
            // Used to build the url
            let (method, path_builder) = sellix_api_models::RequestType::#x_destroy.request_details();
            let path = handlebars::Handlebars::new()
                .render_template(path_builder, &json!({
                    "uniqid": uniqid
                }))
                .expect("unable to parse path");

            // Send it
            self.do_request::<sellix_api_models::RawAPIResponse<()>, Value>(method, &path, None)
                .await
                .and_then(|x| Ok(x.status == sellix_api_models::SellixHttpCode::Ok))
        }
    };
    let delete = if opts.delete.unwrap_or(true) {delete} else {empty};

    // Final output
    let out = quote! {
        impl #ident {
            #client
            #get
            #list
            #create
            #edit
            #delete
        }
    };

    // Return
    TokenStream::from(out)
}