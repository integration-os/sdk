#![deny(clippy::all)]

use std::sync::Arc;

use anyhow::anyhow;
use integrationos_domain::encrypted_access_key::EncryptedAccessKey;
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

#[macro_use]
extern crate napi_derive;

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

#[napi(object)]
#[derive(Clone)]
struct Options {
  pub server_url: String,
}

#[napi]
impl IntegrationOS {
  #[napi(constructor)]
  pub fn new(access_key: String, options: Option<Options>) -> napi::Result<Self> {
    let access_key = EncryptedAccessKey::parse(&access_key)?.to_static();

    let url = Url::parse(
      options
        .as_ref()
        .map(|o| o.server_url.as_str())
        .unwrap_or("https://api.integrationos.com/v1/unified"),
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

  #[napi]
  pub fn customers(&self, connection_key: String) -> CustomerApi {
    CustomerApi {
      client: self.client.clone(),
      connection_key,
      model_name: "customers",
    }
  }
}

#[napi]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Customer {
  pub id: String,
  pub name: String,
  pub age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response<T> {
  pub unified: T,
  pub meta: serde_json::Value,
}

#[napi]
struct CustomerApi {
  client: Arc<Client>,
  connection_key: String,
  model_name: &'static str,
}

#[napi(object)]
struct GetOptions {
  pub id: String,
}

#[napi]
impl CustomerApi {
  #[napi(constructor)]
  pub fn new() -> napi::Result<Self> {
    unimplemented!()
  }

  #[napi(ts_return_type = "Promise<Response<Customer>>")]
  pub async fn get(&self, options: GetOptions) -> napi::Result<serde_json::Value> {
    Ok(
      self
        .client
        .client
        .get(format!(
          "{}/{}/{}",
          self.client.url, self.model_name, options.id
        ))
        .header("x-integrationos-secret", self.client.access_key.to_string())
        .header("x-integrationos-connection-key", &self.connection_key)
        .send()
        .await
        .map_err(|e| anyhow!(e))?
        .json()
        .await
        .map_err(|e| anyhow!(e))?,
    )
  }

  #[napi(ts_return_type = "Promise<Response<Array<Customer>>>")]
  pub async fn list(&self) -> napi::Result<serde_json::Value> {
    Ok(
      self
        .client
        .client
        .get(format!("{}/{}", self.client.url, self.model_name))
        .header("x-integrationos-secret", self.client.access_key.to_string())
        .header("x-integrationos-connection-key", &self.connection_key)
        .send()
        .await
        .map_err(|e| anyhow!(e))?
        .json()
        .await
        .map_err(|e| anyhow!(e))?,
    )
  }
}
