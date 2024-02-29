extern crate napi_build;

use std::collections::HashSet;

use anyhow::Result;
use futures_util::StreamExt;
use integrationos_domain::{
  algebra::extension::StringExt,
  api_model_config::Lang,
  common_model::{CommonEnum, CommonModel, DataType},
  prefix::IdPrefix,
  Id, Store,
};
use mongodb::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let client = Client::with_uri_str("mongodb://localhost:57394/?directConnection=true").await?;
  let db = client.database("events-service");

  let coll = db.collection::<CommonModel>(&Store::CommonModels.to_string());
  let mut common_models = coll.find(None, None).await?;

  let mut cms = String::new();
  let mut apis = String::new();

  let struct_re = regex::Regex::new(r"pub struct").unwrap();

  let mut enums = HashSet::new();

  while let Some(mut cm) = common_models.next().await.transpose()? {
    if cm.name.as_str() == "Collections" {
      continue;
    }

    enums.extend(
      cm.get_enum_fields()
        .into_iter()
        .filter_map(|e| match e.datatype {
          DataType::Enum { options, .. } => Some(CommonEnum {
            id: Id::now(IdPrefix::CommonEnum),
            name: e.name.pascal_case(),
            options: options.unwrap_or_default(),
          }),
          _ => None,
        }),
    );

    let Some(rust) = cm.interface.get_mut(&Lang::Rust) else {
      continue;
    };

    cm.name = cm.name.replace("::", "");

    let rust = struct_re.replace_all(rust, "\n#[napi(object)]$0");

    cms.push_str(rust.as_ref());

    let lowercase_name = cm.name.to_lowercase();

    if cm.primary {
      let lib_fn = format!(
        r#"
  #[napi(ts_return_type = "UnifiedApi<{}>")]
  pub fn {lowercase_name}(&self, connection_key: String) -> UnifiedApi {{
    UnifiedApi::new(
      self.client.clone(),
      connection_key,
      Url::parse(&format!("{{}}/{lowercase_name}", self.client.url)).unwrap(),
    )
  }}"#,
        cm.name
      );
      apis.push_str(&lib_fn);
    }
  }

  let coll = db.collection::<CommonEnum>(&Store::CommonEnums.to_string());
  let mut common_enums = coll.find(None, None).await?;

  let enum_re = regex::Regex::new(r"pub enum").unwrap();

  for mut ce in enums {
    ce.name = ce.name.replace("::", "");
    let rust = ce.as_rust_type();
    let rust = enum_re.replace_all(rust.as_ref(), "\n#[napi(string_enum)]\n$0");
    cms.push_str(rust.as_ref());
  }

  while let Some(mut ce) = common_enums.next().await.transpose()? {
    ce.name = ce.name.replace("::", "");
    let rust = ce.as_rust_type();
    let rust = enum_re.replace_all(rust.as_ref(), "\n#[napi(string_enum)]\n$0");
    cms.push_str(rust.as_ref());
  }

  let _ = tokio::fs::remove_dir_all("src/gen").await;
  tokio::fs::create_dir("src/gen").await?;
  tokio::fs::write("src/gen/mod.rs", cms).await?;
  tokio::fs::write("src/gen/api.rs", apis).await?;

  napi_build::setup();

  Ok(())
}
