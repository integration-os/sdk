use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[napi(object, js_name = "Response<Type>")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Response {
    #[napi(ts_type = "Type")]
    pub unified: Option<serde_json::Value>,
    pub passthrough: Option<serde_json::Value>,
    #[napi(ts_type = "object")]
    pub meta: serde_json::Value,
    pub headers: HashMap<String, String>,
    pub status_code: u16,
}

#[napi(object)]
pub(crate) struct Count {
    pub count: u32,
}

#[napi(object)]
pub(crate) struct Pagination {
    pub limit: Option<u32>,
    pub page_size: u32,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

#[napi(object, js_name = "ListResponse<Type>")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ListResponse {
    #[napi(ts_type = "Array<Type>")]
    pub unified: serde_json::Value,
    pub passthrough: Option<serde_json::Value>,
    #[napi(ts_type = "Pagination")]
    pub pagination: serde_json::Value,
    #[napi(ts_type = "object")]
    pub meta: serde_json::Value,
    pub headers: HashMap<String, String>,
    pub status_code: u16,
}
