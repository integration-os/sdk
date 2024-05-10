use std::sync::Arc;

use http::StatusCode;
use serde::Serialize;
use serde_json::json;
use url::Url;

use crate::*;

#[napi(js_name = "UnifiedApi<Type>")]
pub(crate) struct UnifiedApi {
    client: Arc<Client>,
    connection_key: String,
    url: Url,
}

#[napi(object)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListFilter {
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub updated_after: Option<String>,
    pub updated_before: Option<String>,
    #[serde(with = "serialize_limit")]
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

mod serialize_limit {
    pub fn serialize<S: serde::Serializer>(val: &Option<u32>, ser: S) -> Result<S::Ok, S::Error> {
        match *val {
            Some(ref value) => ser.serialize_some(&value.to_string()),
            None => ser.serialize_none(),
        }
    }
}

#[napi]
impl UnifiedApi {
    pub fn new(client: Arc<Client>, connection_key: String, url: Url) -> Self {
        Self {
            client,
            connection_key,
            url,
        }
    }

    // NAPI won't generate the rest of the methods if there's no constructor
    #[napi(constructor)]
    pub fn _dummy_constructor() {
        unimplemented!()
    }

    #[napi(ts_return_type = "Promise<Response<Type>>")]
    pub async fn create(
        &self,
        #[napi(ts_arg_type = "Type")] object: serde_json::Value,
        options: Option<UnifiedOptions>,
    ) -> napi::Result<Response> {
        let builder = self.client.client.post(self.url.clone()).json(&object);
        Ok(self
            .send(builder, &self.connection_key, options, StatusCode::CREATED)
            .await?)
    }

    #[napi(ts_return_type = "Promise<ListResponse<Type>>")]
    pub async fn list(
        &self,
        filter: Option<ListFilter>,
        options: Option<UnifiedOptions>,
    ) -> napi::Result<ListResponse> {
        let builder = self.client.client.get(self.url.clone()).query(&filter);
        Ok(self
            .send(builder, &self.connection_key, options, StatusCode::OK)
            .await?)
    }

    #[napi(ts_return_type = "Promise<Response<Type>>")]
    pub async fn get(&self, id: String, options: Option<UnifiedOptions>) -> napi::Result<Response> {
        let builder = self.client.client.get(format!("{}/{id}", self.url));
        Ok(self
            .send(builder, &self.connection_key, options, StatusCode::OK)
            .await?)
    }

    #[napi(ts_return_type = "Promise<Response<Type>>")]
    pub async fn update(
        &self,
        id: String,
        #[napi(ts_arg_type = "Type")] object: serde_json::Value,
        options: Option<UnifiedOptions>,
    ) -> napi::Result<Response> {
        let builder = self
            .client
            .client
            .patch(format!("{}/{id}", self.url))
            .json(&object);
        Ok(self
            .send(
                builder,
                &self.connection_key,
                options,
                StatusCode::NO_CONTENT,
            )
            .await?)
    }

    #[napi(ts_return_type = "Promise<Response<Count>>")]
    pub async fn count(&self, options: Option<UnifiedOptions>) -> napi::Result<Response> {
        let builder = self.client.client.get(format!("{}/count", self.url));
        Ok(self
            .send(builder, &self.connection_key, options, StatusCode::OK)
            .await?)
    }

    #[napi(ts_return_type = "Promise<Response<Type>>")]
    pub async fn delete(
        &self,
        id: String,
        delete_options: Option<DeleteOptions>,
        options: Option<UnifiedOptions>,
    ) -> napi::Result<Response> {
        let builder = self
            .client
            .client
            .delete(format!("{}/{id}", self.url))
            .query(&delete_options);
        Ok(self
            .send(
                builder,
                &self.connection_key,
                options,
                StatusCode::NO_CONTENT,
            )
            .await?)
    }

    async fn send<T: for<'a> Deserialize<'a>>(
        &self,
        mut builder: RequestBuilder,
        key: &str,
        options: Option<UnifiedOptions>,
        status_code: StatusCode,
    ) -> anyhow::Result<T> {
        if let Some(options) = options {
            if options.response_passthrough.is_some_and(|p| p) {
                builder = builder.header(PASSTHROUGH_HEADER, "true");
            }

            if let Some(headers) = options.passthrough_headers {
                builder = builder.header(
                    CUSTOM_HEADER,
                    headers
                        .into_iter()
                        .map(|(a, b)| format!("{a}={b}"))
                        .collect::<Vec<_>>()
                        .join(";"),
                );
            }

            if let Some(params) = options.passthrough_query {
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
            .header(SECRET_HEADER, self.client.access_key.to_string())
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
            let mut headers = serde_json::Map::new();
            for (k, v) in res.headers() {
                headers.insert(
                    k.to_string(),
                    http_serde_ext::header_value::serialize(v, serde_json::value::Serializer)
                        .unwrap(),
                );
            }

            match res.json().await {
                Ok(serde_json::Value::Object(mut map)) => {
                    map.insert(
                        RESPONSE_HEADERS_FIELD_NAME.to_string(),
                        serde_json::Value::Object(headers),
                    );
                    map.insert(
                        STATUS_CODE_FIELD_NAME.to_string(),
                        json!(status_code.as_u16()),
                    );

                    Ok(serde_json::from_value(serde_json::Value::Object(map)).unwrap())
                }
                _ => bail!("{{\"error\":\"Invalid response\"}}"),
            }
        }
    }
}
