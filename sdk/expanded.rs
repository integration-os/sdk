`napi` macro expand failed.
#![feature(prelude_import)]
#![deny(clippy::all)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use anyhow::{anyhow, bail};
use integrationos_domain::encrypted_access_key::EncryptedAccessKey;
use reqwest::RequestBuilder;
use serde::Deserialize;
use url::{ParseError, Url};
#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate macros;
mod options {
    use std::collections::HashMap;
    use serde::Serialize;
    pub(crate) struct IntegrationOSOptions {
        pub server_url: String,
    }
    impl napi::bindgen_prelude::TypeName for IntegrationOSOptions {
        fn type_name() -> &'static str {
            "IntegrationOSOptions"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for IntegrationOSOptions {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: IntegrationOSOptions,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { server_url: server_url_ } = val;
            obj.set("serverUrl", server_url_)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for IntegrationOSOptions {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let server_url_: String = obj
                .get("serverUrl")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "serverUrl"),
                        );
                        res
                    },
                ))?;
            let val = Self { server_url: server_url_ };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for IntegrationOSOptions {}
    pub(crate) struct UnifiedOptions {
        pub response_passthrough: Option<bool>,
        pub passthrough_headers: Option<HashMap<String, String>>,
        pub passthrough_query: Option<HashMap<String, String>>,
    }
    impl napi::bindgen_prelude::TypeName for UnifiedOptions {
        fn type_name() -> &'static str {
            "UnifiedOptions"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for UnifiedOptions {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: UnifiedOptions,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                response_passthrough: response_passthrough_,
                passthrough_headers: passthrough_headers_,
                passthrough_query: passthrough_query_,
            } = val;
            if response_passthrough_.is_some() {
                obj.set("responsePassthrough", response_passthrough_)?;
            }
            if passthrough_headers_.is_some() {
                obj.set("passthroughHeaders", passthrough_headers_)?;
            }
            if passthrough_query_.is_some() {
                obj.set("passthroughQuery", passthrough_query_)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for UnifiedOptions {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let response_passthrough_: Option<bool> = obj.get("responsePassthrough")?;
            let passthrough_headers_: Option<HashMap<String, String>> = obj
                .get("passthroughHeaders")?;
            let passthrough_query_: Option<HashMap<String, String>> = obj
                .get("passthroughQuery")?;
            let val = Self {
                response_passthrough: response_passthrough_,
                passthrough_headers: passthrough_headers_,
                passthrough_query: passthrough_query_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for UnifiedOptions {}
    pub(crate) struct DeleteOptions {
        pub modify_token: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for DeleteOptions {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "DeleteOptions",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "modify_token",
                    &self.modify_token,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl napi::bindgen_prelude::TypeName for DeleteOptions {
        fn type_name() -> &'static str {
            "DeleteOptions"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for DeleteOptions {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: DeleteOptions,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { modify_token: modify_token_ } = val;
            if modify_token_.is_some() {
                obj.set("modifyToken", modify_token_)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for DeleteOptions {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let modify_token_: Option<String> = obj.get("modifyToken")?;
            let val = Self {
                modify_token: modify_token_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for DeleteOptions {}
}
mod responses {
    use serde::{Deserialize, Serialize};
    pub(crate) struct Response {
        pub unified: Option<serde_json::Value>,
        pub passthrough: Option<serde_json::Value>,
        pub meta: serde_json::Value,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Response {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Response",
                "unified",
                &self.unified,
                "passthrough",
                &self.passthrough,
                "meta",
                &&self.meta,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Response {
        #[inline]
        fn clone(&self) -> Response {
            Response {
                unified: ::core::clone::Clone::clone(&self.unified),
                passthrough: ::core::clone::Clone::clone(&self.passthrough),
                meta: ::core::clone::Clone::clone(&self.meta),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Response {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Response",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "unified",
                    &self.unified,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "passthrough",
                    &self.passthrough,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "meta",
                    &self.meta,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Response {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "unified" => _serde::__private::Ok(__Field::__field0),
                            "passthrough" => _serde::__private::Ok(__Field::__field1),
                            "meta" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"unified" => _serde::__private::Ok(__Field::__field0),
                            b"passthrough" => _serde::__private::Ok(__Field::__field1),
                            b"meta" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Response>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Response;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Response",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<serde_json::Value>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Response with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<serde_json::Value>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Response with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            serde_json::Value,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Response with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Response {
                            unified: __field0,
                            passthrough: __field1,
                            meta: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Option<serde_json::Value>,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            Option<serde_json::Value>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<serde_json::Value> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "unified",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<serde_json::Value>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "passthrough",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<serde_json::Value>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("meta"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            serde_json::Value,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("unified")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("passthrough")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("meta")?
                            }
                        };
                        _serde::__private::Ok(Response {
                            unified: __field0,
                            passthrough: __field1,
                            meta: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "unified",
                    "passthrough",
                    "meta",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Response",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Response>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl napi::bindgen_prelude::TypeName for Response {
        fn type_name() -> &'static str {
            "Response"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for Response {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Response,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { unified: unified_, passthrough: passthrough_, meta: meta_ } = val;
            if unified_.is_some() {
                obj.set("unified", unified_)?;
            }
            if passthrough_.is_some() {
                obj.set("passthrough", passthrough_)?;
            }
            obj.set("meta", meta_)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for Response {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let unified_: Option<serde_json::Value> = obj.get("unified")?;
            let passthrough_: Option<serde_json::Value> = obj.get("passthrough")?;
            let meta_: serde_json::Value = obj
                .get("meta")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "meta"),
                        );
                        res
                    },
                ))?;
            let val = Self {
                unified: unified_,
                passthrough: passthrough_,
                meta: meta_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for Response {}
    pub(crate) struct Count {
        pub count: u32,
    }
    impl napi::bindgen_prelude::TypeName for Count {
        fn type_name() -> &'static str {
            "Count"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for Count {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Count,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { count: count_ } = val;
            obj.set("count", count_)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for Count {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let count_: u32 = obj
                .get("count")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "count"),
                        );
                        res
                    },
                ))?;
            let val = Self { count: count_ };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for Count {}
    pub(crate) struct Pagination {
        pub limit: Option<u32>,
        pub page_size: u32,
    }
    impl napi::bindgen_prelude::TypeName for Pagination {
        fn type_name() -> &'static str {
            "Pagination"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for Pagination {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Pagination,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { limit: limit_, page_size: page_size_ } = val;
            if limit_.is_some() {
                obj.set("limit", limit_)?;
            }
            obj.set("pageSize", page_size_)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for Pagination {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let limit_: Option<u32> = obj.get("limit")?;
            let page_size_: u32 = obj
                .get("pageSize")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "pageSize"),
                        );
                        res
                    },
                ))?;
            let val = Self {
                limit: limit_,
                page_size: page_size_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for Pagination {}
    pub(crate) struct ListResponse {
        pub unified: serde_json::Value,
        pub passthrough: Option<serde_json::Value>,
        pub pagination: serde_json::Value,
        pub meta: serde_json::Value,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ListResponse {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ListResponse",
                "unified",
                &self.unified,
                "passthrough",
                &self.passthrough,
                "pagination",
                &self.pagination,
                "meta",
                &&self.meta,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ListResponse {
        #[inline]
        fn clone(&self) -> ListResponse {
            ListResponse {
                unified: ::core::clone::Clone::clone(&self.unified),
                passthrough: ::core::clone::Clone::clone(&self.passthrough),
                pagination: ::core::clone::Clone::clone(&self.pagination),
                meta: ::core::clone::Clone::clone(&self.meta),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ListResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ListResponse",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "unified",
                    &self.unified,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "passthrough",
                    &self.passthrough,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "pagination",
                    &self.pagination,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "meta",
                    &self.meta,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ListResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "unified" => _serde::__private::Ok(__Field::__field0),
                            "passthrough" => _serde::__private::Ok(__Field::__field1),
                            "pagination" => _serde::__private::Ok(__Field::__field2),
                            "meta" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"unified" => _serde::__private::Ok(__Field::__field0),
                            b"passthrough" => _serde::__private::Ok(__Field::__field1),
                            b"pagination" => _serde::__private::Ok(__Field::__field2),
                            b"meta" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ListResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ListResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ListResponse",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            serde_json::Value,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ListResponse with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<serde_json::Value>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ListResponse with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            serde_json::Value,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ListResponse with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            serde_json::Value,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct ListResponse with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ListResponse {
                            unified: __field0,
                            passthrough: __field1,
                            pagination: __field2,
                            meta: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<serde_json::Value> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            Option<serde_json::Value>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<serde_json::Value> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<serde_json::Value> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "unified",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            serde_json::Value,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "passthrough",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<serde_json::Value>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "pagination",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            serde_json::Value,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("meta"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            serde_json::Value,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("unified")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("passthrough")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("pagination")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("meta")?
                            }
                        };
                        _serde::__private::Ok(ListResponse {
                            unified: __field0,
                            passthrough: __field1,
                            pagination: __field2,
                            meta: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "unified",
                    "passthrough",
                    "pagination",
                    "meta",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ListResponse",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ListResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl napi::bindgen_prelude::TypeName for ListResponse {
        fn type_name() -> &'static str {
            "ListResponse"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for ListResponse {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: ListResponse,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                unified: unified_,
                passthrough: passthrough_,
                pagination: pagination_,
                meta: meta_,
            } = val;
            obj.set("unified", unified_)?;
            if passthrough_.is_some() {
                obj.set("passthrough", passthrough_)?;
            }
            obj.set("pagination", pagination_)?;
            obj.set("meta", meta_)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for ListResponse {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let unified_: serde_json::Value = obj
                .get("unified")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "unified"),
                        );
                        res
                    },
                ))?;
            let passthrough_: Option<serde_json::Value> = obj.get("passthrough")?;
            let pagination_: serde_json::Value = obj
                .get("pagination")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "pagination"),
                        );
                        res
                    },
                ))?;
            let meta_: serde_json::Value = obj
                .get("meta")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "meta"),
                        );
                        res
                    },
                ))?;
            let val = Self {
                unified: unified_,
                passthrough: passthrough_,
                pagination: pagination_,
                meta: meta_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for ListResponse {}
}
mod unified_api {
    use std::sync::Arc;
    use serde::Serialize;
    use url::Url;
    use crate::*;
    pub(crate) struct UnifiedApi {
        client: Arc<Client>,
        connection_key: String,
        url: Url,
    }
    impl napi::bindgen_prelude::TypeName for UnifiedApi {
        fn type_name() -> &'static str {
            "UnifiedApi"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Function
        }
    }
    impl napi::bindgen_prelude::TypeName for &UnifiedApi {
        fn type_name() -> &'static str {
            "UnifiedApi"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::TypeName for &mut UnifiedApi {
        fn type_name() -> &'static str {
            "UnifiedApi"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for UnifiedApi {
        unsafe fn to_napi_value(
            env: napi::sys::napi_env,
            val: UnifiedApi,
        ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
            if let Some(ctor_ref) = napi::__private::get_class_constructor(
                "UnifiedApi<Type>\0",
            ) {
                let wrapped_value = Box::into_raw(Box::new(val));
                let instance_value = UnifiedApi::new_instance(
                    env,
                    wrapped_value.cast(),
                    ctor_ref,
                )?;
                Ok(instance_value)
            } else {
                Err(
                    napi::bindgen_prelude::Error::new(
                        napi::bindgen_prelude::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get constructor of class `{0}` in `ToNapiValue`",
                                    "UnifiedApi<Type>",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }
    impl napi::bindgen_prelude::ObjectFinalize for UnifiedApi {}
    impl UnifiedApi {
        pub fn instance_of<V: napi::NapiRaw>(
            env: napi::Env,
            value: V,
        ) -> napi::Result<bool> {
            if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
                "UnifiedApi<Type>\0",
            ) {
                let mut ctor = std::ptr::null_mut();
                {
                    let c = unsafe {
                        napi::sys::napi_get_reference_value(
                            env.raw(),
                            ctor_ref,
                            &mut ctor,
                        )
                    };
                    match c {
                        ::napi::sys::Status::napi_ok => Ok(()),
                        _ => {
                            Err(
                                ::napi::Error::new(
                                    ::napi::Status::from(c),
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to get constructor reference of class `{0}`",
                                                "UnifiedApi<Type>\0",
                                            ),
                                        );
                                        res
                                    },
                                ),
                            )
                        }
                    }
                }?;
                let mut is_instance_of = false;
                {
                    let c = unsafe {
                        napi::sys::napi_instanceof(
                            env.raw(),
                            value.raw(),
                            ctor,
                            &mut is_instance_of,
                        )
                    };
                    match c {
                        ::napi::sys::Status::napi_ok => Ok(()),
                        _ => {
                            Err(
                                ::napi::Error::new(
                                    ::napi::Status::from(c),
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to run instanceof for class `{0}`",
                                                "UnifiedApi<Type>\0",
                                            ),
                                        );
                                        res
                                    },
                                ),
                            )
                        }
                    }
                }?;
                Ok(is_instance_of)
            } else {
                Err(
                    napi::Error::new(
                        napi::Status::GenericFailure,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get constructor of class `{0}`",
                                    "UnifiedApi<Type>\0",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }
    impl UnifiedApi {
        pub fn into_reference(
            val: UnifiedApi,
            env: napi::Env,
        ) -> napi::Result<napi::bindgen_prelude::Reference<UnifiedApi>> {
            if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
                "UnifiedApi<Type>\0",
            ) {
                unsafe {
                    let wrapped_value = Box::into_raw(Box::new(val));
                    let instance_value = UnifiedApi::new_instance(
                        env.raw(),
                        wrapped_value.cast(),
                        ctor_ref,
                    )?;
                    {
                        let env = env.raw();
                    }
                    napi::bindgen_prelude::Reference::<
                        UnifiedApi,
                    >::from_value_ptr(wrapped_value.cast(), env.raw())
                }
            } else {
                Err(
                    napi::bindgen_prelude::Error::new(
                        napi::bindgen_prelude::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get constructor of class `{0}`",
                                    "UnifiedApi<Type>",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        }
        pub fn into_instance(
            self,
            env: napi::Env,
        ) -> napi::Result<napi::bindgen_prelude::ClassInstance<UnifiedApi>> {
            if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
                "UnifiedApi<Type>\0",
            ) {
                unsafe {
                    let wrapped_value = Box::leak(Box::new(self));
                    let instance_value = UnifiedApi::new_instance(
                        env.raw(),
                        wrapped_value as *mut _ as *mut std::ffi::c_void,
                        ctor_ref,
                    )?;
                    Ok(
                        napi::bindgen_prelude::ClassInstance::<
                            UnifiedApi,
                        >::new(instance_value, wrapped_value),
                    )
                }
            } else {
                Err(
                    napi::bindgen_prelude::Error::new(
                        napi::bindgen_prelude::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get constructor of class `{0}`",
                                    "UnifiedApi<Type>",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        }
        unsafe fn new_instance(
            env: napi::sys::napi_env,
            wrapped_value: *mut std::ffi::c_void,
            ctor_ref: napi::sys::napi_ref,
        ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
            let mut ctor = std::ptr::null_mut();
            {
                let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get constructor reference of class `{0}`",
                                            "UnifiedApi<Type>",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            let mut result = std::ptr::null_mut();
            napi::__private::___CALL_FROM_FACTORY
                .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
            {
                let c = napi::sys::napi_new_instance(
                    env,
                    ctor,
                    0,
                    std::ptr::null_mut(),
                    &mut result,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to construct class `{0}`",
                                            "UnifiedApi<Type>",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            napi::__private::___CALL_FROM_FACTORY
                .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
            let mut object_ref = std::ptr::null_mut();
            let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
            let finalize_callbacks_ptr = std::rc::Rc::into_raw(
                std::rc::Rc::new(std::cell::Cell::new(Box::into_raw(initial_finalize))),
            );
            {
                let c = napi::sys::napi_wrap(
                    env,
                    result,
                    wrapped_value,
                    Some(napi::bindgen_prelude::raw_finalize_unchecked::<UnifiedApi>),
                    std::ptr::null_mut(),
                    &mut object_ref,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to wrap native object of class `{0}`",
                                            "UnifiedApi<Type>",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            napi::bindgen_prelude::Reference::<
                UnifiedApi,
            >::add_ref(
                env,
                wrapped_value,
                (wrapped_value, object_ref, finalize_callbacks_ptr),
            );
            Ok(result)
        }
    }
    impl napi::bindgen_prelude::FromNapiRef for UnifiedApi {
        unsafe fn from_napi_ref(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<&'static Self> {
            let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
            {
                let c = napi::bindgen_prelude::sys::napi_unwrap(
                    env,
                    napi_val,
                    &mut wrapped_val,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to recover `{0}` type from napi value",
                                            "UnifiedApi",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            Ok(&*(wrapped_val as *const UnifiedApi))
        }
    }
    impl napi::bindgen_prelude::FromNapiMutRef for UnifiedApi {
        unsafe fn from_napi_mut_ref(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<&'static mut Self> {
            let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
            {
                let c = napi::bindgen_prelude::sys::napi_unwrap(
                    env,
                    napi_val,
                    &mut wrapped_val,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to recover `{0}` type from napi value",
                                            "UnifiedApi",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            Ok(&mut *(wrapped_val as *mut UnifiedApi))
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for &UnifiedApi {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for &mut UnifiedApi {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for &UnifiedApi {
        unsafe fn validate(
            env: napi::sys::napi_env,
            napi_val: napi::sys::napi_value,
        ) -> napi::Result<napi::sys::napi_value> {
            if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
                "UnifiedApi\0",
            ) {
                let mut ctor = std::ptr::null_mut();
                {
                    let c = napi::sys::napi_get_reference_value(
                        env,
                        ctor_ref,
                        &mut ctor,
                    );
                    match c {
                        ::napi::sys::Status::napi_ok => Ok(()),
                        _ => {
                            Err(
                                ::napi::Error::new(
                                    ::napi::Status::from(c),
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to get constructor reference of class `{0}`",
                                                "UnifiedApi",
                                            ),
                                        );
                                        res
                                    },
                                ),
                            )
                        }
                    }
                }?;
                let mut is_instance_of = false;
                {
                    let c = napi::sys::napi_instanceof(
                        env,
                        napi_val,
                        ctor,
                        &mut is_instance_of,
                    );
                    match c {
                        ::napi::sys::Status::napi_ok => Ok(()),
                        _ => {
                            Err(
                                ::napi::Error::new(
                                    ::napi::Status::from(c),
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to get external value of class `{0}`",
                                                "UnifiedApi",
                                            ),
                                        );
                                        res
                                    },
                                ),
                            )
                        }
                    }
                }?;
                if is_instance_of {
                    Ok(std::ptr::null_mut())
                } else {
                    Err(
                        napi::Error::new(
                            napi::Status::InvalidArg,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Value is not instanceof class `{0}`",
                                        "UnifiedApi",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            } else {
                Err(
                    napi::Error::new(
                        napi::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get constructor of class `{0}`",
                                    "UnifiedApi",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for &mut UnifiedApi {
        unsafe fn validate(
            env: napi::sys::napi_env,
            napi_val: napi::sys::napi_value,
        ) -> napi::Result<napi::sys::napi_value> {
            if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
                "UnifiedApi\0",
            ) {
                let mut ctor = std::ptr::null_mut();
                {
                    let c = napi::sys::napi_get_reference_value(
                        env,
                        ctor_ref,
                        &mut ctor,
                    );
                    match c {
                        ::napi::sys::Status::napi_ok => Ok(()),
                        _ => {
                            Err(
                                ::napi::Error::new(
                                    ::napi::Status::from(c),
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to get constructor reference of class `{0}`",
                                                "UnifiedApi",
                                            ),
                                        );
                                        res
                                    },
                                ),
                            )
                        }
                    }
                }?;
                let mut is_instance_of = false;
                {
                    let c = napi::sys::napi_instanceof(
                        env,
                        napi_val,
                        ctor,
                        &mut is_instance_of,
                    );
                    match c {
                        ::napi::sys::Status::napi_ok => Ok(()),
                        _ => {
                            Err(
                                ::napi::Error::new(
                                    ::napi::Status::from(c),
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to get external value of class `{0}`",
                                                "UnifiedApi",
                                            ),
                                        );
                                        res
                                    },
                                ),
                            )
                        }
                    }
                }?;
                if is_instance_of {
                    Ok(std::ptr::null_mut())
                } else {
                    Err(
                        napi::Error::new(
                            napi::Status::InvalidArg,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Value is not instanceof class `{0}`",
                                        "UnifiedApi",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            } else {
                Err(
                    napi::Error::new(
                        napi::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get constructor of class `{0}`",
                                    "UnifiedApi",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }
    #[allow(clippy::all)]
    #[allow(non_snake_case)]
    mod __napi_helper__UnifiedApi {
        use std::ptr;
        use super::*;
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
        extern fn __napi_register__UnifiedApi_struct_7() {
            napi::__private::register_class(
                "UnifiedApi",
                None,
                "UnifiedApi<Type>\0",
                ::alloc::vec::Vec::new(),
            );
        }
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        #[link_section = ".init_array"]
        static __napi_register__UnifiedApi_struct_7___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __napi_register__UnifiedApi_struct_7___rust_ctor___ctor() -> usize {
                __napi_register__UnifiedApi_struct_7();
                0
            }
            __napi_register__UnifiedApi_struct_7___rust_ctor___ctor
        };
    }
    pub(crate) struct ListFilter {
        pub created_after: Option<String>,
        pub created_before: Option<String>,
        pub updated_after: Option<String>,
        pub updated_before: Option<String>,
        #[serde(with = "serialize_limit")]
        pub limit: Option<u32>,
        pub cursor: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ListFilter {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ListFilter",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "created_after",
                    &self.created_after,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "created_before",
                    &self.created_before,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updated_after",
                    &self.updated_after,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updated_before",
                    &self.updated_before,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "limit",
                    {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a Option<u32>,),
                            phantom: _serde::__private::PhantomData<ListFilter>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                serialize_limit::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.limit,),
                            phantom: _serde::__private::PhantomData::<ListFilter>,
                        }
                    },
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "cursor",
                    &self.cursor,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl napi::bindgen_prelude::TypeName for ListFilter {
        fn type_name() -> &'static str {
            "ListFilter"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for ListFilter {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: ListFilter,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                created_after: created_after_,
                created_before: created_before_,
                updated_after: updated_after_,
                updated_before: updated_before_,
                limit: limit_,
                cursor: cursor_,
            } = val;
            if created_after_.is_some() {
                obj.set("createdAfter", created_after_)?;
            }
            if created_before_.is_some() {
                obj.set("createdBefore", created_before_)?;
            }
            if updated_after_.is_some() {
                obj.set("updatedAfter", updated_after_)?;
            }
            if updated_before_.is_some() {
                obj.set("updatedBefore", updated_before_)?;
            }
            if limit_.is_some() {
                obj.set("limit", limit_)?;
            }
            if cursor_.is_some() {
                obj.set("cursor", cursor_)?;
            }
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for ListFilter {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let created_after_: Option<String> = obj.get("createdAfter")?;
            let created_before_: Option<String> = obj.get("createdBefore")?;
            let updated_after_: Option<String> = obj.get("updatedAfter")?;
            let updated_before_: Option<String> = obj.get("updatedBefore")?;
            let limit_: Option<u32> = obj.get("limit")?;
            let cursor_: Option<String> = obj.get("cursor")?;
            let val = Self {
                created_after: created_after_,
                created_before: created_before_,
                updated_after: updated_after_,
                updated_before: updated_before_,
                limit: limit_,
                cursor: cursor_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for ListFilter {}
    mod serialize_limit {
        pub fn serialize<S: serde::Serializer>(
            val: &Option<u32>,
            ser: S,
        ) -> Result<S::Ok, S::Error> {
            match *val {
                Some(ref value) => ser.serialize_some(&value.to_string()),
                None => ser.serialize_none(),
            }
        }
    }
    impl UnifiedApi {
        pub fn new(client: Arc<Client>, connection_key: String, url: Url) -> Self {
            Self {
                client,
                connection_key,
                url,
            }
        }
        pub fn _dummy_constructor() {
            ::core::panicking::panic("not implemented")
        }
        pub async fn create(
            &self,
            object: serde_json::Value,
            options: Option<UnifiedOptions>,
        ) -> napi::Result<Response> {
            let builder = self.client.client.post(self.url.clone()).json(&object);
            Ok(self.send(builder, &self.connection_key, options).await?)
        }
        pub async fn list(
            &self,
            filter: Option<ListFilter>,
            options: Option<UnifiedOptions>,
        ) -> napi::Result<ListResponse> {
            let builder = self.client.client.get(self.url.clone()).query(&filter);
            Ok(self.send(builder, &self.connection_key, options).await?)
        }
        pub async fn get(
            &self,
            id: String,
            options: Option<UnifiedOptions>,
        ) -> napi::Result<Response> {
            let builder = self
                .client
                .client
                .get({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/{1}", self.url, id),
                    );
                    res
                });
            Ok(self.send(builder, &self.connection_key, options).await?)
        }
        pub async fn update(
            &self,
            id: String,
            object: serde_json::Value,
            options: Option<UnifiedOptions>,
        ) -> napi::Result<Response> {
            let builder = self
                .client
                .client
                .patch({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/{1}", self.url, id),
                    );
                    res
                })
                .json(&object);
            Ok(self.send(builder, &self.connection_key, options).await?)
        }
        pub async fn count(
            &self,
            options: Option<UnifiedOptions>,
        ) -> napi::Result<Response> {
            let builder = self
                .client
                .client
                .get({
                    let res = ::alloc::fmt::format(format_args!("{0}/count", self.url));
                    res
                });
            Ok(self.send(builder, &self.connection_key, options).await?)
        }
        pub async fn delete(
            &self,
            id: String,
            delete_options: Option<DeleteOptions>,
            options: Option<UnifiedOptions>,
        ) -> napi::Result<Response> {
            let builder = self
                .client
                .client
                .delete({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/{1}", self.url, id),
                    );
                    res
                })
                .json(&delete_options);
            Ok(self.send(builder, &self.connection_key, options).await?)
        }
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
                    builder = builder
                        .header(
                            CUSTOM_HEADER,
                            headers
                                .into_iter()
                                .map(|(a, b)| {
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}={1}", a, b),
                                    );
                                    res
                                })
                                .collect::<Vec<_>>()
                                .join(";"),
                        );
                }
                if let Some(params) = options.passthrough_headers {
                    builder = builder
                        .query(
                            &[
                                (
                                    CUSTOM_QUERY,
                                    params
                                        .into_iter()
                                        .map(|(a, b)| {
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0}={1}", a, b),
                                            );
                                            res
                                        })
                                        .collect::<Vec<_>>()
                                        .join("&"),
                                ),
                            ],
                        );
                }
            }
            let res = builder
                .header(SECRET_HEADER, self.client.access_key.to_string())
                .header(CONNECTION_HEADER, key)
                .send()
                .await
                .map_err(|e| ::anyhow::__private::must_use({
                    use ::anyhow::__private::kind::*;
                    let error = match e {
                        error => (&error).anyhow_kind().new(error),
                    };
                    error
                }))?;
            let status = res.status();
            if !status.is_success() {
                match res.json::<serde_json::Value>().await {
                    Ok(json) => {
                        return ::anyhow::__private::Err({
                            use ::anyhow::__private::kind::*;
                            let error = match json {
                                error => (&error).anyhow_kind().new(error),
                            };
                            error
                        });
                    }
                    Err(_) => {
                        return ::anyhow::__private::Err({
                            let error = ::anyhow::__private::format_err(
                                format_args!("{{\"error\":\"Invalid response\"}}"),
                            );
                            error
                        });
                    }
                }
            } else {
                match res.json().await {
                    Ok(json) => return Ok(json),
                    Err(_) => {
                        return ::anyhow::__private::Err({
                            let error = ::anyhow::__private::format_err(
                                format_args!("{{\"error\":\"Invalid response\"}}"),
                            );
                            error
                        });
                    }
                }
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    mod __napi_impl_helper__UnifiedApi__0 {
        use super::*;
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn __napi___dummy_constructor(
            env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                if napi::__private::___CALL_FROM_FACTORY
                    .with(|inner| inner.load(std::sync::atomic::Ordering::Relaxed))
                {
                    return std::ptr::null_mut();
                }
                napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None, false)
                    .and_then(|mut cb| {
                        napi::bindgen_prelude::within_runtime_if_available(move || {
                            let _ret = { UnifiedApi::_dummy_constructor() };
                            <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                env,
                                (),
                            )
                        })
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn __napi__create(
            env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                napi::bindgen_prelude::CallbackInfo::<2usize>::new(env, cb, None, false)
                    .and_then(|mut cb| {
                        struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
                        impl NapiRefContainer {
                            fn drop(self, env: napi::sys::napi_env) {
                                for r in self.0.into_iter() {
                                    match (
                                        &unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                    match (
                                        &unsafe { napi::sys::napi_delete_reference(env, r) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                }
                            }
                        }
                        unsafe impl Send for NapiRefContainer {}
                        unsafe impl Sync for NapiRefContainer {}
                        let _make_ref = |
                            a: ::std::ptr::NonNull<
                                napi::bindgen_prelude::sys::napi_value__,
                            >|
                        {
                            let mut node_ref = ::std::mem::MaybeUninit::uninit();
                            {
                                let c = unsafe {
                                    napi::bindgen_prelude::sys::napi_create_reference(
                                        env,
                                        a.as_ptr(),
                                        1,
                                        node_ref.as_mut_ptr(),
                                    )
                                };
                                match c {
                                    ::napi::sys::Status::napi_ok => Ok(()),
                                    _ => {
                                        Err(
                                            ::napi::Error::new(
                                                ::napi::Status::from(c),
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("failed to create napi ref"),
                                                    );
                                                    res
                                                },
                                            ),
                                        )
                                    }
                                }
                            }?;
                            Ok::<
                                napi::sys::napi_ref,
                                napi::Error,
                            >(unsafe { node_ref.assume_init() })
                        };
                        let mut _args_array = [::std::ptr::null_mut::<
                            napi::bindgen_prelude::sys::napi_ref__,
                        >(); 1usize];
                        let mut _arg_write_index = 0;
                        _args_array[_arg_write_index] = _make_ref(
                            ::std::ptr::NonNull::new(cb.this)
                                .ok_or_else(|| napi::Error::new(
                                    napi::Status::InvalidArg,
                                    "referenced ptr is null".to_owned(),
                                ))?,
                        )?;
                        _arg_write_index += 1;
                        let _args_ref = NapiRefContainer(_args_array);
                        let this_ptr = unsafe { cb.unwrap_raw::<UnifiedApi>()? };
                        let this: &UnifiedApi = Box::leak(Box::from_raw(this_ptr));
                        let arg0 = {
                            <serde_json::Value as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(0usize),
                            )?
                        };
                        let arg1 = {
                            <Option<
                                UnifiedOptions,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(1usize),
                            )?
                        };
                        napi::bindgen_prelude::execute_tokio_future(
                            env,
                            async move { this.create(arg0, arg1).await },
                            move |env, _ret| {
                                _args_ref.drop(env);
                                <Response as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                    env,
                                    _ret,
                                )
                            },
                        )
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn __napi__list(
            env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                napi::bindgen_prelude::CallbackInfo::<2usize>::new(env, cb, None, false)
                    .and_then(|mut cb| {
                        struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
                        impl NapiRefContainer {
                            fn drop(self, env: napi::sys::napi_env) {
                                for r in self.0.into_iter() {
                                    match (
                                        &unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                    match (
                                        &unsafe { napi::sys::napi_delete_reference(env, r) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                }
                            }
                        }
                        unsafe impl Send for NapiRefContainer {}
                        unsafe impl Sync for NapiRefContainer {}
                        let _make_ref = |
                            a: ::std::ptr::NonNull<
                                napi::bindgen_prelude::sys::napi_value__,
                            >|
                        {
                            let mut node_ref = ::std::mem::MaybeUninit::uninit();
                            {
                                let c = unsafe {
                                    napi::bindgen_prelude::sys::napi_create_reference(
                                        env,
                                        a.as_ptr(),
                                        1,
                                        node_ref.as_mut_ptr(),
                                    )
                                };
                                match c {
                                    ::napi::sys::Status::napi_ok => Ok(()),
                                    _ => {
                                        Err(
                                            ::napi::Error::new(
                                                ::napi::Status::from(c),
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("failed to create napi ref"),
                                                    );
                                                    res
                                                },
                                            ),
                                        )
                                    }
                                }
                            }?;
                            Ok::<
                                napi::sys::napi_ref,
                                napi::Error,
                            >(unsafe { node_ref.assume_init() })
                        };
                        let mut _args_array = [::std::ptr::null_mut::<
                            napi::bindgen_prelude::sys::napi_ref__,
                        >(); 1usize];
                        let mut _arg_write_index = 0;
                        _args_array[_arg_write_index] = _make_ref(
                            ::std::ptr::NonNull::new(cb.this)
                                .ok_or_else(|| napi::Error::new(
                                    napi::Status::InvalidArg,
                                    "referenced ptr is null".to_owned(),
                                ))?,
                        )?;
                        _arg_write_index += 1;
                        let _args_ref = NapiRefContainer(_args_array);
                        let this_ptr = unsafe { cb.unwrap_raw::<UnifiedApi>()? };
                        let this: &UnifiedApi = Box::leak(Box::from_raw(this_ptr));
                        let arg0 = {
                            <Option<
                                ListFilter,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(0usize),
                            )?
                        };
                        let arg1 = {
                            <Option<
                                UnifiedOptions,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(1usize),
                            )?
                        };
                        napi::bindgen_prelude::execute_tokio_future(
                            env,
                            async move { this.list(arg0, arg1).await },
                            move |env, _ret| {
                                _args_ref.drop(env);
                                <ListResponse as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                    env,
                                    _ret,
                                )
                            },
                        )
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn __napi__get(
            env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                napi::bindgen_prelude::CallbackInfo::<2usize>::new(env, cb, None, false)
                    .and_then(|mut cb| {
                        struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
                        impl NapiRefContainer {
                            fn drop(self, env: napi::sys::napi_env) {
                                for r in self.0.into_iter() {
                                    match (
                                        &unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                    match (
                                        &unsafe { napi::sys::napi_delete_reference(env, r) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                }
                            }
                        }
                        unsafe impl Send for NapiRefContainer {}
                        unsafe impl Sync for NapiRefContainer {}
                        let _make_ref = |
                            a: ::std::ptr::NonNull<
                                napi::bindgen_prelude::sys::napi_value__,
                            >|
                        {
                            let mut node_ref = ::std::mem::MaybeUninit::uninit();
                            {
                                let c = unsafe {
                                    napi::bindgen_prelude::sys::napi_create_reference(
                                        env,
                                        a.as_ptr(),
                                        1,
                                        node_ref.as_mut_ptr(),
                                    )
                                };
                                match c {
                                    ::napi::sys::Status::napi_ok => Ok(()),
                                    _ => {
                                        Err(
                                            ::napi::Error::new(
                                                ::napi::Status::from(c),
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("failed to create napi ref"),
                                                    );
                                                    res
                                                },
                                            ),
                                        )
                                    }
                                }
                            }?;
                            Ok::<
                                napi::sys::napi_ref,
                                napi::Error,
                            >(unsafe { node_ref.assume_init() })
                        };
                        let mut _args_array = [::std::ptr::null_mut::<
                            napi::bindgen_prelude::sys::napi_ref__,
                        >(); 1usize];
                        let mut _arg_write_index = 0;
                        _args_array[_arg_write_index] = _make_ref(
                            ::std::ptr::NonNull::new(cb.this)
                                .ok_or_else(|| napi::Error::new(
                                    napi::Status::InvalidArg,
                                    "referenced ptr is null".to_owned(),
                                ))?,
                        )?;
                        _arg_write_index += 1;
                        let _args_ref = NapiRefContainer(_args_array);
                        let this_ptr = unsafe { cb.unwrap_raw::<UnifiedApi>()? };
                        let this: &UnifiedApi = Box::leak(Box::from_raw(this_ptr));
                        let arg0 = {
                            <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(0usize),
                            )?
                        };
                        let arg1 = {
                            <Option<
                                UnifiedOptions,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(1usize),
                            )?
                        };
                        napi::bindgen_prelude::execute_tokio_future(
                            env,
                            async move { this.get(arg0, arg1).await },
                            move |env, _ret| {
                                _args_ref.drop(env);
                                <Response as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                    env,
                                    _ret,
                                )
                            },
                        )
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn __napi__update(
            env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                napi::bindgen_prelude::CallbackInfo::<3usize>::new(env, cb, None, false)
                    .and_then(|mut cb| {
                        struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
                        impl NapiRefContainer {
                            fn drop(self, env: napi::sys::napi_env) {
                                for r in self.0.into_iter() {
                                    match (
                                        &unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                    match (
                                        &unsafe { napi::sys::napi_delete_reference(env, r) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                }
                            }
                        }
                        unsafe impl Send for NapiRefContainer {}
                        unsafe impl Sync for NapiRefContainer {}
                        let _make_ref = |
                            a: ::std::ptr::NonNull<
                                napi::bindgen_prelude::sys::napi_value__,
                            >|
                        {
                            let mut node_ref = ::std::mem::MaybeUninit::uninit();
                            {
                                let c = unsafe {
                                    napi::bindgen_prelude::sys::napi_create_reference(
                                        env,
                                        a.as_ptr(),
                                        1,
                                        node_ref.as_mut_ptr(),
                                    )
                                };
                                match c {
                                    ::napi::sys::Status::napi_ok => Ok(()),
                                    _ => {
                                        Err(
                                            ::napi::Error::new(
                                                ::napi::Status::from(c),
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("failed to create napi ref"),
                                                    );
                                                    res
                                                },
                                            ),
                                        )
                                    }
                                }
                            }?;
                            Ok::<
                                napi::sys::napi_ref,
                                napi::Error,
                            >(unsafe { node_ref.assume_init() })
                        };
                        let mut _args_array = [::std::ptr::null_mut::<
                            napi::bindgen_prelude::sys::napi_ref__,
                        >(); 1usize];
                        let mut _arg_write_index = 0;
                        _args_array[_arg_write_index] = _make_ref(
                            ::std::ptr::NonNull::new(cb.this)
                                .ok_or_else(|| napi::Error::new(
                                    napi::Status::InvalidArg,
                                    "referenced ptr is null".to_owned(),
                                ))?,
                        )?;
                        _arg_write_index += 1;
                        let _args_ref = NapiRefContainer(_args_array);
                        let this_ptr = unsafe { cb.unwrap_raw::<UnifiedApi>()? };
                        let this: &UnifiedApi = Box::leak(Box::from_raw(this_ptr));
                        let arg0 = {
                            <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(0usize),
                            )?
                        };
                        let arg1 = {
                            <serde_json::Value as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(1usize),
                            )?
                        };
                        let arg2 = {
                            <Option<
                                UnifiedOptions,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(2usize),
                            )?
                        };
                        napi::bindgen_prelude::execute_tokio_future(
                            env,
                            async move { this.update(arg0, arg1, arg2).await },
                            move |env, _ret| {
                                _args_ref.drop(env);
                                <Response as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                    env,
                                    _ret,
                                )
                            },
                        )
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn __napi__count(
            env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                napi::bindgen_prelude::CallbackInfo::<1usize>::new(env, cb, None, false)
                    .and_then(|mut cb| {
                        struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
                        impl NapiRefContainer {
                            fn drop(self, env: napi::sys::napi_env) {
                                for r in self.0.into_iter() {
                                    match (
                                        &unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                    match (
                                        &unsafe { napi::sys::napi_delete_reference(env, r) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                }
                            }
                        }
                        unsafe impl Send for NapiRefContainer {}
                        unsafe impl Sync for NapiRefContainer {}
                        let _make_ref = |
                            a: ::std::ptr::NonNull<
                                napi::bindgen_prelude::sys::napi_value__,
                            >|
                        {
                            let mut node_ref = ::std::mem::MaybeUninit::uninit();
                            {
                                let c = unsafe {
                                    napi::bindgen_prelude::sys::napi_create_reference(
                                        env,
                                        a.as_ptr(),
                                        1,
                                        node_ref.as_mut_ptr(),
                                    )
                                };
                                match c {
                                    ::napi::sys::Status::napi_ok => Ok(()),
                                    _ => {
                                        Err(
                                            ::napi::Error::new(
                                                ::napi::Status::from(c),
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("failed to create napi ref"),
                                                    );
                                                    res
                                                },
                                            ),
                                        )
                                    }
                                }
                            }?;
                            Ok::<
                                napi::sys::napi_ref,
                                napi::Error,
                            >(unsafe { node_ref.assume_init() })
                        };
                        let mut _args_array = [::std::ptr::null_mut::<
                            napi::bindgen_prelude::sys::napi_ref__,
                        >(); 1usize];
                        let mut _arg_write_index = 0;
                        _args_array[_arg_write_index] = _make_ref(
                            ::std::ptr::NonNull::new(cb.this)
                                .ok_or_else(|| napi::Error::new(
                                    napi::Status::InvalidArg,
                                    "referenced ptr is null".to_owned(),
                                ))?,
                        )?;
                        _arg_write_index += 1;
                        let _args_ref = NapiRefContainer(_args_array);
                        let this_ptr = unsafe { cb.unwrap_raw::<UnifiedApi>()? };
                        let this: &UnifiedApi = Box::leak(Box::from_raw(this_ptr));
                        let arg0 = {
                            <Option<
                                UnifiedOptions,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(0usize),
                            )?
                        };
                        napi::bindgen_prelude::execute_tokio_future(
                            env,
                            async move { this.count(arg0).await },
                            move |env, _ret| {
                                _args_ref.drop(env);
                                <Response as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                    env,
                                    _ret,
                                )
                            },
                        )
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn __napi__delete(
            env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                napi::bindgen_prelude::CallbackInfo::<3usize>::new(env, cb, None, false)
                    .and_then(|mut cb| {
                        struct NapiRefContainer([napi::sys::napi_ref; 1usize]);
                        impl NapiRefContainer {
                            fn drop(self, env: napi::sys::napi_env) {
                                for r in self.0.into_iter() {
                                    match (
                                        &unsafe { napi::sys::napi_reference_unref(env, r, &mut 0) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                    match (
                                        &unsafe { napi::sys::napi_delete_reference(env, r) },
                                        &napi::sys::Status::napi_ok,
                                    ) {
                                        (left_val, right_val) => {
                                            if !(*left_val == *right_val) {
                                                let kind = ::core::panicking::AssertKind::Eq;
                                                ::core::panicking::assert_failed(
                                                    kind,
                                                    &*left_val,
                                                    &*right_val,
                                                    ::core::option::Option::Some(
                                                        format_args!("failed to delete napi ref"),
                                                    ),
                                                );
                                            }
                                        }
                                    };
                                }
                            }
                        }
                        unsafe impl Send for NapiRefContainer {}
                        unsafe impl Sync for NapiRefContainer {}
                        let _make_ref = |
                            a: ::std::ptr::NonNull<
                                napi::bindgen_prelude::sys::napi_value__,
                            >|
                        {
                            let mut node_ref = ::std::mem::MaybeUninit::uninit();
                            {
                                let c = unsafe {
                                    napi::bindgen_prelude::sys::napi_create_reference(
                                        env,
                                        a.as_ptr(),
                                        1,
                                        node_ref.as_mut_ptr(),
                                    )
                                };
                                match c {
                                    ::napi::sys::Status::napi_ok => Ok(()),
                                    _ => {
                                        Err(
                                            ::napi::Error::new(
                                                ::napi::Status::from(c),
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("failed to create napi ref"),
                                                    );
                                                    res
                                                },
                                            ),
                                        )
                                    }
                                }
                            }?;
                            Ok::<
                                napi::sys::napi_ref,
                                napi::Error,
                            >(unsafe { node_ref.assume_init() })
                        };
                        let mut _args_array = [::std::ptr::null_mut::<
                            napi::bindgen_prelude::sys::napi_ref__,
                        >(); 1usize];
                        let mut _arg_write_index = 0;
                        _args_array[_arg_write_index] = _make_ref(
                            ::std::ptr::NonNull::new(cb.this)
                                .ok_or_else(|| napi::Error::new(
                                    napi::Status::InvalidArg,
                                    "referenced ptr is null".to_owned(),
                                ))?,
                        )?;
                        _arg_write_index += 1;
                        let _args_ref = NapiRefContainer(_args_array);
                        let this_ptr = unsafe { cb.unwrap_raw::<UnifiedApi>()? };
                        let this: &UnifiedApi = Box::leak(Box::from_raw(this_ptr));
                        let arg0 = {
                            <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(0usize),
                            )?
                        };
                        let arg1 = {
                            <Option<
                                DeleteOptions,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(1usize),
                            )?
                        };
                        let arg2 = {
                            <Option<
                                UnifiedOptions,
                            > as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                                env,
                                cb.get_arg(2usize),
                            )?
                        };
                        napi::bindgen_prelude::execute_tokio_future(
                            env,
                            async move { this.delete(arg0, arg1, arg2).await },
                            move |env, _ret| {
                                _args_ref.drop(env);
                                <Response as napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                    env,
                                    _ret,
                                )
                            },
                        )
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }
        #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
        extern fn __napi_register__UnifiedApi_impl_16() {
            napi::__private::register_class(
                "UnifiedApi",
                None,
                "UnifiedApi<Type>\0",
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        napi::bindgen_prelude::Property::new("constructor")
                            .unwrap()
                            .with_property_attributes(
                                napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                    .unwrap(),
                            )
                            .with_ctor(__napi___dummy_constructor),
                        napi::bindgen_prelude::Property::new("count")
                            .unwrap()
                            .with_property_attributes(
                                napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                    .unwrap(),
                            )
                            .with_method(__napi__count),
                        napi::bindgen_prelude::Property::new("create")
                            .unwrap()
                            .with_property_attributes(
                                napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                    .unwrap(),
                            )
                            .with_method(__napi__create),
                        napi::bindgen_prelude::Property::new("delete")
                            .unwrap()
                            .with_property_attributes(
                                napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                    .unwrap(),
                            )
                            .with_method(__napi__delete),
                        napi::bindgen_prelude::Property::new("get")
                            .unwrap()
                            .with_property_attributes(
                                napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                    .unwrap(),
                            )
                            .with_method(__napi__get),
                        napi::bindgen_prelude::Property::new("list")
                            .unwrap()
                            .with_property_attributes(
                                napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                    .unwrap(),
                            )
                            .with_method(__napi__list),
                        napi::bindgen_prelude::Property::new("update")
                            .unwrap()
                            .with_property_attributes(
                                napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                    .unwrap(),
                            )
                            .with_method(__napi__update),
                    ]),
                ),
            );
        }
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        #[link_section = ".init_array"]
        static __napi_register__UnifiedApi_impl_16___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __napi_register__UnifiedApi_impl_16___rust_ctor___ctor() -> usize {
                __napi_register__UnifiedApi_impl_16();
                0
            }
            __napi_register__UnifiedApi_impl_16___rust_ctor___ctor
        };
    }
}
use options::*;
use responses::*;
use unified_api::*;
const DEFAULT_URL: &str = "https://api.integrationos.com/v1/unified";
const PASSTHROUGH_HEADER: &str = "x-integrationos-enable-passthrough";
const SECRET_HEADER: &str = "x-integrationos-secret";
const CONNECTION_HEADER: &str = "x-integrationos-connection-key";
const CUSTOM_HEADER: &str = "x-integrationos-passthrough";
const CUSTOM_QUERY: &str = "integrationOSPassthrough";
struct Client {
    access_key: EncryptedAccessKey<'static>,
    url: Url,
    client: reqwest::Client,
}
#[automatically_derived]
impl ::core::fmt::Debug for Client {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Client",
            "access_key",
            &self.access_key,
            "url",
            &self.url,
            "client",
            &&self.client,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Client {
    #[inline]
    fn clone(&self) -> Client {
        Client {
            access_key: ::core::clone::Clone::clone(&self.access_key),
            url: ::core::clone::Clone::clone(&self.url),
            client: ::core::clone::Clone::clone(&self.client),
        }
    }
}
struct IntegrationOS {
    client: Arc<Client>,
}
impl napi::bindgen_prelude::TypeName for IntegrationOS {
    fn type_name() -> &'static str {
        "IntegrationOS"
    }
    fn value_type() -> napi::ValueType {
        napi::ValueType::Function
    }
}
impl napi::bindgen_prelude::TypeName for &IntegrationOS {
    fn type_name() -> &'static str {
        "IntegrationOS"
    }
    fn value_type() -> napi::ValueType {
        napi::ValueType::Object
    }
}
impl napi::bindgen_prelude::TypeName for &mut IntegrationOS {
    fn type_name() -> &'static str {
        "IntegrationOS"
    }
    fn value_type() -> napi::ValueType {
        napi::ValueType::Object
    }
}
impl napi::bindgen_prelude::ToNapiValue for IntegrationOS {
    unsafe fn to_napi_value(
        env: napi::sys::napi_env,
        val: IntegrationOS,
    ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
        if let Some(ctor_ref) = napi::__private::get_class_constructor(
            "IntegrationOS\0",
        ) {
            let wrapped_value = Box::into_raw(Box::new(val));
            let instance_value = IntegrationOS::new_instance(
                env,
                wrapped_value.cast(),
                ctor_ref,
            )?;
            Ok(instance_value)
        } else {
            Err(
                napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}` in `ToNapiValue`",
                                "IntegrationOS",
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
impl napi::bindgen_prelude::ObjectFinalize for IntegrationOS {}
impl IntegrationOS {
    pub fn instance_of<V: napi::NapiRaw>(
        env: napi::Env,
        value: V,
    ) -> napi::Result<bool> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
            "IntegrationOS\0",
        ) {
            let mut ctor = std::ptr::null_mut();
            {
                let c = unsafe {
                    napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor)
                };
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get constructor reference of class `{0}`",
                                            "IntegrationOS\0",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            let mut is_instance_of = false;
            {
                let c = unsafe {
                    napi::sys::napi_instanceof(
                        env.raw(),
                        value.raw(),
                        ctor,
                        &mut is_instance_of,
                    )
                };
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to run instanceof for class `{0}`",
                                            "IntegrationOS\0",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            Ok(is_instance_of)
        } else {
            Err(
                napi::Error::new(
                    napi::Status::GenericFailure,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`",
                                "IntegrationOS\0",
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
impl IntegrationOS {
    pub fn into_reference(
        val: IntegrationOS,
        env: napi::Env,
    ) -> napi::Result<napi::bindgen_prelude::Reference<IntegrationOS>> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
            "IntegrationOS\0",
        ) {
            unsafe {
                let wrapped_value = Box::into_raw(Box::new(val));
                let instance_value = IntegrationOS::new_instance(
                    env.raw(),
                    wrapped_value.cast(),
                    ctor_ref,
                )?;
                {
                    let env = env.raw();
                }
                napi::bindgen_prelude::Reference::<
                    IntegrationOS,
                >::from_value_ptr(wrapped_value.cast(), env.raw())
            }
        } else {
            Err(
                napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`",
                                "IntegrationOS",
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
    pub fn into_instance(
        self,
        env: napi::Env,
    ) -> napi::Result<napi::bindgen_prelude::ClassInstance<IntegrationOS>> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
            "IntegrationOS\0",
        ) {
            unsafe {
                let wrapped_value = Box::leak(Box::new(self));
                let instance_value = IntegrationOS::new_instance(
                    env.raw(),
                    wrapped_value as *mut _ as *mut std::ffi::c_void,
                    ctor_ref,
                )?;
                Ok(
                    napi::bindgen_prelude::ClassInstance::<
                        IntegrationOS,
                    >::new(instance_value, wrapped_value),
                )
            }
        } else {
            Err(
                napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`",
                                "IntegrationOS",
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
    unsafe fn new_instance(
        env: napi::sys::napi_env,
        wrapped_value: *mut std::ffi::c_void,
        ctor_ref: napi::sys::napi_ref,
    ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
        let mut ctor = std::ptr::null_mut();
        {
            let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to get constructor reference of class `{0}`",
                                        "IntegrationOS",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        let mut result = std::ptr::null_mut();
        napi::__private::___CALL_FROM_FACTORY
            .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
        {
            let c = napi::sys::napi_new_instance(
                env,
                ctor,
                0,
                std::ptr::null_mut(),
                &mut result,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to construct class `{0}`",
                                        "IntegrationOS",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        napi::__private::___CALL_FROM_FACTORY
            .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
        let mut object_ref = std::ptr::null_mut();
        let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
        let finalize_callbacks_ptr = std::rc::Rc::into_raw(
            std::rc::Rc::new(std::cell::Cell::new(Box::into_raw(initial_finalize))),
        );
        {
            let c = napi::sys::napi_wrap(
                env,
                result,
                wrapped_value,
                Some(napi::bindgen_prelude::raw_finalize_unchecked::<IntegrationOS>),
                std::ptr::null_mut(),
                &mut object_ref,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to wrap native object of class `{0}`",
                                        "IntegrationOS",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        napi::bindgen_prelude::Reference::<
            IntegrationOS,
        >::add_ref(
            env,
            wrapped_value,
            (wrapped_value, object_ref, finalize_callbacks_ptr),
        );
        Ok(result)
    }
}
impl napi::bindgen_prelude::FromNapiRef for IntegrationOS {
    unsafe fn from_napi_ref(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<&'static Self> {
        let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_unwrap(
                env,
                napi_val,
                &mut wrapped_val,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to recover `{0}` type from napi value",
                                        "IntegrationOS",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        Ok(&*(wrapped_val as *const IntegrationOS))
    }
}
impl napi::bindgen_prelude::FromNapiMutRef for IntegrationOS {
    unsafe fn from_napi_mut_ref(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<&'static mut Self> {
        let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_unwrap(
                env,
                napi_val,
                &mut wrapped_val,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to recover `{0}` type from napi value",
                                        "IntegrationOS",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        Ok(&mut *(wrapped_val as *mut IntegrationOS))
    }
}
impl napi::bindgen_prelude::FromNapiValue for &IntegrationOS {
    unsafe fn from_napi_value(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<Self> {
        napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
    }
}
impl napi::bindgen_prelude::FromNapiValue for &mut IntegrationOS {
    unsafe fn from_napi_value(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<Self> {
        napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
    }
}
impl napi::bindgen_prelude::ValidateNapiValue for &IntegrationOS {
    unsafe fn validate(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> napi::Result<napi::sys::napi_value> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
            "IntegrationOS\0",
        ) {
            let mut ctor = std::ptr::null_mut();
            {
                let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get constructor reference of class `{0}`",
                                            "IntegrationOS",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            let mut is_instance_of = false;
            {
                let c = napi::sys::napi_instanceof(
                    env,
                    napi_val,
                    ctor,
                    &mut is_instance_of,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get external value of class `{0}`",
                                            "IntegrationOS",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            if is_instance_of {
                Ok(std::ptr::null_mut())
            } else {
                Err(
                    napi::Error::new(
                        napi::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Value is not instanceof class `{0}`",
                                    "IntegrationOS",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        } else {
            Err(
                napi::Error::new(
                    napi::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`",
                                "IntegrationOS",
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut IntegrationOS {
    unsafe fn validate(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> napi::Result<napi::sys::napi_value> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor(
            "IntegrationOS\0",
        ) {
            let mut ctor = std::ptr::null_mut();
            {
                let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get constructor reference of class `{0}`",
                                            "IntegrationOS",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            let mut is_instance_of = false;
            {
                let c = napi::sys::napi_instanceof(
                    env,
                    napi_val,
                    ctor,
                    &mut is_instance_of,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get external value of class `{0}`",
                                            "IntegrationOS",
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            if is_instance_of {
                Ok(std::ptr::null_mut())
            } else {
                Err(
                    napi::Error::new(
                        napi::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Value is not instanceof class `{0}`",
                                    "IntegrationOS",
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        } else {
            Err(
                napi::Error::new(
                    napi::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`",
                                "IntegrationOS",
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__IntegrationOS {
    use std::ptr;
    use super::*;
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    #[cfg(all(not(test), not(feature = "noop"), not(target_family = "wasm")))]
    extern fn __napi_register__IntegrationOS_struct_17() {
        napi::__private::register_class(
            "IntegrationOS",
            None,
            "IntegrationOS\0",
            ::alloc::vec::Vec::new(),
        );
    }
    #[used]
    #[allow(non_upper_case_globals, non_snake_case)]
    #[doc(hidden)]
    #[link_section = ".init_array"]
    static __napi_register__IntegrationOS_struct_17___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
        #[allow(non_snake_case)]
        #[link_section = ".text.startup"]
        unsafe extern "C" fn __napi_register__IntegrationOS_struct_17___rust_ctor___ctor() -> usize {
            __napi_register__IntegrationOS_struct_17();
            0
        }
        __napi_register__IntegrationOS_struct_17___rust_ctor___ctor
    };
}
