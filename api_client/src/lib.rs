pub mod namespaces;
pub mod errors;
pub mod auth;
pub mod values;

use reqwest::Client;

pub struct MakoApiClient  {
    base_url: String,
    client: Client,
    auth_provider: Box<dyn auth::AuthProvider>,
}

impl MakoApiClient {
    pub fn new(base_url: String, auth_provider: Box<dyn auth::AuthProvider>) -> Self {
        Self {
            base_url,
            client: Client::new(),
            auth_provider,
        }
    }

    pub fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        log::debug!("Making request: {} {}", method, path);

        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), path);
        let mut req = self.client.request(method, &url);

        let auth_header = self.auth_provider.get_auth_header();
        req = req.header("Authorization", auth_header);

        req
    }

    pub fn namespaces(&self) -> namespaces::NamespaceClient {
        namespaces::NamespaceClient::new(self)
    }
}

