use std::collections::HashMap;

use serde::Serialize;

#[napi(object)]
pub(crate) struct IntegrationOSOptions {
    pub server_url: String,
}

#[napi(object)]
pub(crate) struct UnifiedOptions {
    pub response_passthrough: Option<bool>,
    pub passthrough_headers: Option<HashMap<String, String>>,
    pub passthrough_query: Option<HashMap<String, String>>,
}

#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteOptions {
    pub modify_token: Option<String>,
}
