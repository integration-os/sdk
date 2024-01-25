#![deny(clippy::all)]

use std::str::FromStr;

use anyhow::Result;
use integrationos_domain::{
  common::{
    access_key_data::AccessKeyData, access_key_prefix::AccessKeyPrefix,
    encrypted_data::PASSWORD_LENGTH, environment::Environment, event_type::EventType, AccessKey,
  },
  id::{prefix::IdPrefix, Id},
};
use rand::Rng;

#[macro_use]
extern crate napi_derive;

#[napi(object, js_name = "AccessKey")]
struct JsAccessKey {
  pub environment: String,
  pub event_type: String,
  pub version: u32,
  pub data: AccessKeyData,
}

fn validate_password(password: &str) -> Result<&[u8; PASSWORD_LENGTH]> {
  if password.len() != PASSWORD_LENGTH {
    return Err(anyhow::anyhow!(
      "Password must be 32 characters in length. Given password is {} characters long",
      password.len()
    ));
  }
  Ok(password.as_bytes().try_into()?)
}

#[napi]
#[allow(dead_code)]
fn encode_access_key(access_key: JsAccessKey, password: String) -> Result<String> {
  let password = validate_password(&password)?;
  let prefix = AccessKeyPrefix {
    environment: Environment::from_str(&access_key.environment)?,
    event_type: EventType::try_from(&*access_key.event_type)?,
    version: access_key.version,
  };
  let access_key = AccessKey {
    prefix,
    data: access_key.data,
  };
  let iv = rand::thread_rng().gen::<[u8; 16]>();
  let encrypted_access_key = access_key.encode(password, &iv)?;
  Ok(encrypted_access_key.to_string())
}

#[napi]
#[allow(dead_code)]
fn decode_access_key(access_key: String, password: String) -> Result<JsAccessKey> {
  let password = validate_password(&password)?;
  let key = AccessKey::parse_str(&access_key, password)?;
  Ok(JsAccessKey {
    environment: key.prefix.environment.to_string(),
    event_type: key.prefix.event_type.to_string(),
    version: key.prefix.version,
    data: key.data,
  })
}

#[napi]
#[allow(dead_code)]
fn generate_id(prefix: String) -> Result<String> {
  let prefix = IdPrefix::try_from(prefix.as_ref())?;
  let id = Id::now(prefix);
  Ok(id.to_string())
}
