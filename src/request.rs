use std::collections::HashMap;
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

#[derive(Default)]
#[napi(object)]
pub struct RequestInit {
  pub method: Option<HttpMethod>,
  pub headers: Option<HashMap<String, String>>,
  pub body: Option<Vec<u8>>,
}