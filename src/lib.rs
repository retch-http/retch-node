use retch::retcher::{RequestOptions, Retcher, RetcherConfig};
use napi_derive::napi;

mod response;
mod request;

use self::response::RetchResponse;

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

#[napi(js_name="Retcher")]
pub struct RetcherWrapper {
  inner: Retcher,
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

