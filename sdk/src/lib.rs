#![deny(clippy::all)]
#![allow(dead_code)]

use std::sync::Arc;

use anyhow::{anyhow, bail};
use reqwest::RequestBuilder;
use serde::Deserialize;
use url::{ParseError, Url};

#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate macros;

mod options;
mod responses;
mod unified_api;
use options::*;
use responses::*;
use unified_api::*;

const DEFAULT_URL: &str = "https://api.integrationos.com/v1/unified";
const PASSTHROUGH_HEADER: &str = "x-integrationos-enable-passthrough";
const SECRET_HEADER: &str = "x-integrationos-secret";
const CONNECTION_HEADER: &str = "x-integrationos-connection-key";
const CUSTOM_HEADER: &str = "x-integrationos-passthrough";
const CUSTOM_QUERY: &str = "integrationOSPassthrough";

#[derive(Debug, Clone)]
struct Client {
    access_key: String,
    url: Url,
    client: reqwest::Client,
}

#[napi(js_name = "IntegrationOS")]
struct IntegrationOS {
    client: Arc<Client>,
}

#[unified_api]
impl IntegrationOS {
    #[napi(constructor)]
    pub fn new(access_key: String, options: Option<IntegrationOSOptions>) -> napi::Result<Self> {
        let url = Url::parse(
            options
                .as_ref()
                .map(|o| {
                    let url = o.server_url.as_str();
                    if url.ends_with("/") {
                        &url[..url.len() - 1]
                    } else {
                        url
                    }
                })
                .unwrap_or(DEFAULT_URL),
        )
        .map_err(|e: ParseError| anyhow!(e))?;

        let client = reqwest::Client::default();

        Ok(Self {
            client: Arc::new(Client {
                access_key,
                url,
                client,
            }),
        })
    }
}

make_common_models!();
