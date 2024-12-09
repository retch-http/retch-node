use std::collections::HashMap;

use napi::{bindgen_prelude::Buffer, Env, JsFunction, JsObject, JsString, JsUnknown};
use reqwest::Response;
use retch::retcher::{ Retcher, RetcherConfig};
use napi_derive::napi;

#[napi]
pub enum Browser {
  Chrome,
  Firefox,
}

impl Into<retch::Browser> for Browser {
  fn into(self) -> retch::Browser {
    match self {
      Browser::Chrome => retch::Browser::Chrome,
      Browser::Firefox => retch::Browser::Firefox,
    }
  }
}

#[napi(object)]
struct RetcherOptions {
  pub browser: Option<Browser>,
  pub ignore_tls_errors: Option<bool>,
  pub vanilla_fallback: Option<bool>,
  pub proxy_url: Option<String>,
}

impl Into<RetcherConfig> for RetcherOptions {
  fn into(self) -> RetcherConfig {
    let mut config = RetcherConfig::default();
    if let Some(browser) = self.browser {
      config = config.with_browser(browser.into());
    }
    if let Some(ignore_tls_errors) = self.ignore_tls_errors {
      config = config.with_ignore_tls_errors(ignore_tls_errors);
    }
    if let Some(vanilla_fallback) = self.vanilla_fallback {
      config = config.with_fallback_to_vanilla(vanilla_fallback);
    }
    if let Some(proxy_url) = self.proxy_url {
      config = config.with_proxy(proxy_url);
    }
    config
  }
}

#[napi]
struct RetchResponse {
  bytes: Vec<u8>,
  pub status: u16,
  pub status_text: String,
  pub headers: HashMap<String, String>,
  pub ok: bool,
}

#[napi]
impl RetchResponse {
  async fn from(response: Response) -> Self {
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

#[napi(js_name="Retcher")]
pub struct RetcherWrapper {
  inner: Retcher,
}

#[napi]
impl RetcherWrapper {
    #[napi(constructor)]
    pub fn new(options: RetcherOptions) -> Self {
      let config: RetcherConfig = options.into();
        Self {
          inner: config.build(),
        }
    }

    #[napi]
    pub async fn fetch(&self, url: String) -> RetchResponse {
      let response = self.inner.get(url, None).await.unwrap();
      
      RetchResponse::from(response).await
    }
}

