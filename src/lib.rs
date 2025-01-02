use retch::retcher::{RequestOptions, Retcher, RetcherConfig};
use napi_derive::napi;

mod response;
mod request;
mod retcher_builder;

use request::{HttpMethod, RequestInit};
use retcher_builder::RetcherOptions;
use self::response::RetchResponse;


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

