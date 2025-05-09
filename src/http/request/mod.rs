pub mod get;
pub mod post;

use super::{Endpoint, UrlQueryParams};
use crate::error::{Error, Result};
use crate::vndb::Token;
use http::Method;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Response as RawResponse};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::{LazyLock, Weak};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio::task::spawn;
use tokio::time::{Duration, sleep};

pub const BASE_URL: &str = "https://api.vndb.org/kana";
const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

static HTTP: LazyLock<Client> = LazyLock::new(|| {
  Client::builder()
    .use_rustls_tls()
    .https_only(true)
    .build()
    .expect("failed to create http client")
});

#[bon::builder]
pub(super) async fn request<Body>(
  #[builder(start_fn)] endpoint: Endpoint,
  method: Method,
  semaphore: Weak<Semaphore>,
  query: Option<UrlQueryParams>,
  body: Option<&Body>,
  token: Option<&Token>,
  delay: Option<Duration>,
  timeout: Option<Duration>,
  user_agent: Option<&str>,
) -> Result<RawResponse>
where
  Body: Serialize + ?Sized,
{
  let permit = semaphore
    .upgrade()
    .unwrap()
    .acquire_owned()
    .await
    .map_err(|_| Error::ClientClosed)?;

  let mut url = endpoint.url();
  if let Some(query) = query {
    url.query_pairs_mut().extend_pairs(query.0);
  }

  let mut request = HTTP.request(method, url);

  if let Some(body) = body {
    request = request.header(CONTENT_TYPE, "application/json");
    request = request.json(body);
  }

  if let Some(token) = token {
    request = request.header(AUTHORIZATION, token.to_header());
  }

  if let Some(timeout) = timeout {
    request = request.timeout(timeout);
  }

  if let Some(user_agent) = user_agent {
    request = request.header(USER_AGENT, user_agent);
  } else {
    request = request.header(USER_AGENT, DEFAULT_USER_AGENT);
  }

  let response = request.send().await?.error_for_status()?;

  if let Some(delay) = delay {
    delay_drop(permit, delay);
  } else {
    drop(permit);
  }

  Ok(response)
}

#[bon::builder]
pub(super) async fn request_json<Body, Json>(
  #[builder(start_fn)] endpoint: Endpoint,
  semaphore: Weak<Semaphore>,
  method: Method,
  query: Option<UrlQueryParams>,
  body: Option<&Body>,
  token: Option<&Token>,
  delay: Option<Duration>,
  timeout: Option<Duration>,
  user_agent: Option<&str>,
) -> Result<Json>
where
  Body: Serialize + ?Sized,
  Json: DeserializeOwned,
{
  request(endpoint)
    .method(method)
    .semaphore(semaphore)
    .maybe_query(query)
    .maybe_body(body)
    .maybe_token(token)
    .maybe_delay(delay)
    .maybe_timeout(timeout)
    .maybe_user_agent(user_agent)
    .call()
    .await?
    .json()
    .await
    .map_err(Into::into)
}

fn delay_drop(permit: OwnedSemaphorePermit, duration: Duration) {
  spawn(async move {
    sleep(duration).await;
    drop(permit);
  });
}
