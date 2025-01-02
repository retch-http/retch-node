use std::time::Duration;

use retch::retcher::RetcherBuilder;
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
pub struct RetcherOptions {
  pub browser: Option<Browser>,
  pub ignore_tls_errors: Option<bool>,
  pub vanilla_fallback: Option<bool>,
  pub proxy_url: Option<String>,
  /// Default timeout for this Retcher instance in milliseconds.
  pub timeout: Option<u32>,
}

impl Into<RetcherBuilder> for RetcherOptions {
  fn into(self) -> RetcherBuilder {
    let mut config = RetcherBuilder::default();
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
    if let Some(timeout) = self.timeout {
      config = config.with_default_timeout(Duration::from_millis(timeout.into()));
    }
    config
  }
}
