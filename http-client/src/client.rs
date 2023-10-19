use anyhow::Result;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::endpoint::{self, StreamBuilder};

#[derive(Debug, Clone)]
pub struct Client {
    pub host: String,
    pub config: Option<Config>,
    pub client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub sync_interval: u32,
}

impl Client {
    /// Create a client using the specified server.
    pub fn new(host: impl Into<String>, config: Option<Config>) -> Result<Client> {
        // common api headers
        let headers = HeaderMap::new();
        // common client settings for REST endpoint reqwests
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            host: host.into(),
            config,
            client,
        })
    }
    /// Return the host associated with this client.
    pub fn get_host(&self) -> &str {
        &self.host
    }
    /// Return the config associated with this client.
    pub fn get_config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
    /// Return the stream api
    pub fn builder<T>(&self, builder: Option<T>) -> endpoint::RequestEndpoint<T> {
        endpoint::RequestEndpoint {
            setup: self,
            builder: builder,
        }
    }
}
