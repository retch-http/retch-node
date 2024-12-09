use std::collections::HashMap;

use napi::{bindgen_prelude::Buffer, Env, JsFunction, JsObject, JsString, JsUnknown};
use reqwest::Response;
use retch::retcher::{ RequestOptions, Retcher, RetcherConfig};
use napi_derive::napi;

#[napi(string_enum)]
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

#[derive(Default)]
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

#[napi(js_name="Retcher")]
pub struct RetcherWrapper {
  inner: Retcher,
}

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

#[napi]
impl RetcherWrapper {
    #[napi(constructor)]
    pub fn new(options: Option<RetcherOptions>) -> Self {
      let config: RetcherConfig = options.unwrap_or_default().into();
        Self {
          inner: config.build(),
        }
    }

    #[napi]
    pub async fn fetch(&self, url: String, request_init: Option<RequestInit>) -> RetchResponse {
      let request_options = Some(RequestOptions {
        headers: request_init.as_ref().and_then(|init| init.headers.as_ref()).cloned().unwrap_or_default(),
      });

      let body = request_init.as_ref().and_then(|init| init.body.as_ref()).cloned();

      let response = match request_init.unwrap_or_default().method.unwrap_or_default() {
        HttpMethod::GET => self.inner.get(url, request_options).await,
        HttpMethod::POST => self.inner.post(url, body, request_options).await,
        HttpMethod::PUT => self.inner.put(url, body, request_options).await,
        HttpMethod::DELETE => self.inner.delete(url, request_options).await,
        HttpMethod::PATCH => self.inner.patch(url, body, request_options).await,
        HttpMethod::HEAD => self.inner.head(url, request_options).await,
        HttpMethod::OPTIONS => self.inner.options(url, request_options).await,
      };

      let response = response.expect("fatal: Couldn't fetch the URL");
      RetchResponse::from(response).await
    }
}

