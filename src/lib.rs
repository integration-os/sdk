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
  #[napi(ts_return_type = "UnifiedApi<Companies>")]
  pub fn companies(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/companies", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Leads>")]
  pub fn leads(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/leads", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Notes>")]
  pub fn notes(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/notes", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Users>")]
  pub fn users(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/users", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Opportunities>")]
  pub fn opportunities(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/opportunities", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Orders>")]
  pub fn orders(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/orders", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Products>")]
  pub fn products(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/products", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Tasks>")]
  pub fn tasks(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/tasks", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Jobs>")]
  pub fn jobs(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/jobs", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Contacts>")]
  pub fn contacts(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/contacts", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Candidates>")]
  pub fn candidates(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/candidates", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Tickets>")]
  pub fn tickets(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/tickets", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Invoices>")]
  pub fn invoices(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/invoices", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<JournalEntries>")]
  pub fn journalentries(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/journalentries", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<IncomeStatements>")]
  pub fn incomestatements(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/incomestatements", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<BalanceSheets>")]
  pub fn balancesheets(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/balancesheets", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Vendors>")]
  pub fn vendors(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/vendors", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Bills>")]
  pub fn bills(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/bills", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Payments>")]
  pub fn payments(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/payments", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<PurchaseOrders>")]
  pub fn purchaseorders(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/purchaseorders", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Accounts>")]
  pub fn accounts(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/accounts", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Transactions>")]
  pub fn transactions(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/transactions", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Expenses>")]
  pub fn expenses(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/expenses", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<CreditNotes>")]
  pub fn creditnotes(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/creditnotes", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<TaxRates>")]
  pub fn taxrates(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/taxrates", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Chats>")]
  pub fn chats(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/chats", self.client.url)).unwrap(),
    )
  }
  #[napi(ts_return_type = "UnifiedApi<Messages>")]
  pub fn messages(&self, connection_key: String) -> UnifiedApi {
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{}/messages", self.client.url)).unwrap(),
    )
  }
}

pub mod gen;
