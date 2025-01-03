use std::collections::HashMap;
use napi::{bindgen_prelude::Buffer, Either};
use napi_derive::napi;

#[derive(Default)]
#[napi(string_enum)]
pub enum HttpMethod {
  #[default]
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS,
}

pub(crate) fn serialize_body(body: Either<String, Buffer>) -> Vec<u8> {
  match body {
    Either::A(string) => string.into_bytes(),
    Either::B(buffer) => buffer.into(),
  }
}

#[derive(Default)]
#[napi(object)]
pub struct RequestInit {
  pub method: Option<HttpMethod>,
  pub headers: Option<HashMap<String, String>>,
  pub body: Option<Either<String, Buffer>>,
  /// Request timeout in milliseconds. Overrides the Retcher-wide timeout option.
  pub timeout: Option<u32>,
  /// Force the request to use HTTP/3. If the server doesn't expect HTTP/3, the request will fail.
  pub force_http3: Option<bool>,
}