extern crate proc_macro;

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
use mongodb::{bson, Client};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ImplItemFn, ItemImpl};

#[proc_macro]
pub fn make_common_models(_item: TokenStream) -> TokenStream {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { get_common_models().await }).unwrap()
}

async fn get_common_models() -> Result<TokenStream> {
    let client = Client::with_uri_str("mongodb://localhost:57394/?directConnection=true").await?;
    let db = client.database("events-service");

    let coll = db.collection::<CommonModel>(&Store::CommonModels.to_string());
    let mut common_models = coll.find(None, None).await?;

    let mut enums = HashSet::new();

    let mut output = String::new();

    while let Some(cm) = common_models.next().await.transpose()? {
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

        if cm.name.as_str() == "Collections" {
            continue;
        }

        let Some(rust) = cm.interface.get(&Lang::Rust) else {
            continue;
        };

        let napi_attr = format!("#[napi(object, js_name = {})]", cm.name.replace("::", ""));

        output.push_str(&napi_attr);
        output.push_str(rust);
    }

    let coll = db.collection::<CommonEnum>(&Store::CommonEnums.to_string());
    let mut common_enums = coll.find(None, None).await?;

    for ce in enums {
        let napi_attr = format!(
            "#[napi(string_enum, js_name = {})]",
            ce.name.replace("::", "")
        );

        let rust = ce.as_rust_type();
        output.push_str(&napi_attr);
        output.push_str(&rust);
    }

    while let Some(ce) = common_enums.next().await.transpose()? {
        let napi_attr = format!(
            "#[napi(string_enum, js_name = {})]",
            ce.name.replace("::", "")
        );

        let rust = ce.as_rust_type();
        output.push_str(&napi_attr);
        output.push_str(&rust);
    }

    Ok(output.parse().unwrap())
}

#[proc_macro_attribute]
pub fn unified_api(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    let rt = tokio::runtime::Runtime::new().expect("unable to create runtime");
    rt.block_on(async { get_primary_models(&mut input).await })
        .expect("primary models to be added to impl item");

    quote! {
        #[napi]
        #input
    }
    .into()
}

async fn get_primary_models(impl_block: &mut ItemImpl) -> Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:57394/?directConnection=true").await?;
    let db = client.database("events-service");

    let coll = db.collection::<CommonModel>(&Store::CommonModels.to_string());
    let mut common_models = coll.find(bson::doc! { "primary": true }, None).await?;

    while let Some(mut cm) = common_models.next().await.transpose()? {
        cm.name = cm.name.replace("::", "");
        let name = &cm.name;
        let lowercase_name = cm.name.to_lowercase();

        let return_type = format!("UnifiedApi<{name}>");
        let url_format_string = format!("{{}}/{lowercase_name}");
        let fn_name: proc_macro2::TokenStream = lowercase_name
            .parse()
            .expect("lowercase name to parse to tokenstream");

        let api_fn = quote! {
            #[napi(ts_return_type = #return_type)]
            pub fn #fn_name(&self, connection_key: String) -> UnifiedApi {
                UnifiedApi::new(
                    self.client.clone(),
                    connection_key,
                    Url::parse(&format!(#url_format_string, self.client.url)).unwrap(),
                )
            }
        };

        let parsed_fn: ImplItemFn =
            syn::parse2(api_fn).expect("unable to parse quoted fn as impl fn");

        impl_block.items.push(ImplItem::Fn(parsed_fn));
    }

    Ok(())
}
