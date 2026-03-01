pub mod namespaces;
pub mod errors;

use reqwest::Client;

pub struct MakoApiClient  {
    base_url: String,
    client: Client,
}

impl MakoApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub fn namespaces(&self) -> namespaces::NamespaceClient {
        namespaces::NamespaceClient::new(self)
    }
}

