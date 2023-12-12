use std::time::Duration;

use crate::client::Client;

pub struct ClientOptions {
    pub(crate) api_endpoint: String,
    pub(crate) api_key: String,
    pub(crate) timeout: Duration,
}

impl ClientOptions {
    pub fn new(api_key: String, api_endpoint: String, timeout: Duration) -> ClientOptions {
        ClientOptions {
            api_endpoint,
            api_key,
            timeout,
        }
    }

    pub fn api_endpoint(&mut self, api_endpoint: String) -> &mut Self {
        self.api_endpoint = api_endpoint.to_string();
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Client {
        Client::new(self)
    }
}
