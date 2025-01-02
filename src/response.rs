use std::collections::HashMap;

use napi::{bindgen_prelude::Buffer, Env, JsFunction, JsObject, JsString, JsUnknown};
use napi_derive::napi;
use reqwest::Response;

#[napi]
pub(crate) struct RetchResponse {
  bytes: Vec<u8>,
  pub status: u16,
  pub status_text: String,
  pub headers: HashMap<String, String>,
  pub ok: bool,
}

#[napi]
impl RetchResponse {
  // not the Trait From - this method needs to be async.
  pub(crate) async fn from(response: Response) -> Self {
    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("").to_string();
    let headers = response.headers().iter().map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string())).collect();
    let ok = response.status().is_success();
    let bytes = response.bytes().await.unwrap().to_vec();
    Self {
      bytes,
      status,
      status_text,
      headers,
      ok,
    }
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    self.bytes.clone().into()
  }

  #[napi]
  pub fn text(&self) -> String {
    // Support non-UTF-8 encodings (from the content-type header, the http-equiv meta tag, etc.)
    String::from_utf8_lossy(&self.bytes).to_string()
  }

  #[napi(ts_return_type="any")]
  pub fn json(&self, env: Env) -> JsUnknown {
    let text = self.text();

    env.get_global().and_then(
      |global| global.get_named_property::<JsObject>("JSON")
    ).and_then(
      |json| json.get_named_property::<JsFunction>("parse")
    ).expect("fatal: Couldn't get JSON.parse")
    .call::<JsString>(
      None,
      &[env.create_string_from_std(text).expect("Couldn't create JS string from the response text")],
    ).expect("fatal: Couldn't parse JSON")
  }
}