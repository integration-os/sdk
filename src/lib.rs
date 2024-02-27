#![deny(clippy::all)]

use std::sync::Arc;

use anyhow::{anyhow, bail};
use integrationos_domain::encrypted_access_key::EncryptedAccessKey;
use reqwest::RequestBuilder;
use serde::Deserialize;
use url::{ParseError, Url};

#[macro_use]
extern crate napi_derive;

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
  access_key: EncryptedAccessKey<'static>,
  url: Url,
  client: reqwest::Client,
}

#[napi(js_name = "IntegrationOS")]
struct IntegrationOS {
  client: Arc<Client>,
}

#[napi]
impl IntegrationOS {
  #[napi(constructor)]
  pub fn new(access_key: String, options: Option<IntegrationOSOptions>) -> napi::Result<Self> {
    let access_key = EncryptedAccessKey::parse(&access_key)
      .map_err(|e| anyhow!(e))?
      .to_static();

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

  #[napi(ts_return_type = "UnifiedApi<Customers>")]
  pub fn customers(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/customers", self.client.url)).unwrap(),
    )
  }
}

impl Client {
  async fn send<T: for<'a> Deserialize<'a>>(
    &self,
    mut builder: RequestBuilder,
    key: &str,
    options: Option<UnifiedOptions>,
  ) -> anyhow::Result<T> {
    if let Some(mut options) = options {
      if options.response_passthrough.is_some_and(|p| p) {
        builder = builder.header(PASSTHROUGH_HEADER, "true");
      }

      if let Some(headers) = options.passthrough_headers.take() {
        builder = builder.header(
          CUSTOM_HEADER,
          headers
            .into_iter()
            .map(|(a, b)| format!("{a}={b}"))
            .collect::<Vec<_>>()
            .join(";"),
        );
      }

      if let Some(params) = options.passthrough_headers {
        builder = builder.query(&[(
          CUSTOM_QUERY,
          params
            .into_iter()
            .map(|(a, b)| format!("{a}={b}"))
            .collect::<Vec<_>>()
            .join("&"),
        )]);
      }
    }

    let res = builder
      .header(SECRET_HEADER, self.access_key.to_string())
      .header(CONNECTION_HEADER, key)
      .send()
      .await
      .map_err(|e| anyhow!(e))?;

    let status = res.status();
    if !status.is_success() {
      match res.json::<serde_json::Value>().await {
        Ok(json) => bail!(json),
        Err(_) => bail!("{{\"error\":\"Invalid response\"}}"),
      }
    } else {
      match res.json().await {
        Ok(json) => return Ok(json),
        Err(_) => bail!("{{\"error\":\"Invalid response\"}}"),
      }
    }
  }
}
