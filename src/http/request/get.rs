use super::{request, request_json};
use crate::error::{Error, Result};
use crate::http::{Endpoint, FieldSet, UrlQueryParams};
use crate::make_request;
use crate::model::prelude::*;
use crate::vndb::{Token, Vndb};
use http::Method;
use reqwest::Response as RawResponse;
use serde::de::DeserializeOwned;
use std::sync::Weak;
use tokio::sync::Semaphore;
use tokio::time::Duration;

pub struct Get {
  vndb: Weak<Vndb>,
}

impl Get {
  pub fn new(vndb: Weak<Vndb>) -> Self {
    Self { vndb }
  }

  pub async fn auth_info(&self) -> Result<AuthInfo> {
    let vndb = Vndb::upgrade(&self.vndb)?;
    if vndb.token.is_none() {
      return Err(Error::TokenNeeded);
    }

    make_request!(vndb, get_json(Endpoint::AuthInfo))
  }

  pub async fn schema(&self) -> Result<Schema> {
    let vndb = Vndb::upgrade(&self.vndb)?;
    make_request!(vndb, get_json(Endpoint::Schema))
  }

  pub async fn stats(&self) -> Result<Stats> {
    let vndb = Vndb::upgrade(&self.vndb)?;
    make_request!(vndb, get_json(Endpoint::Stats))
  }

  /// Search for a user by their id or username.
  pub async fn user<UserQuery, Field>(&self, user: UserQuery, fields: Field) -> Result<Users>
  where
    UserQuery: Into<UserUrlQuery>,
    Field: Into<FieldSet<UserField>>,
  {
    let mut query = user.into().into_query();
    if query.is_empty() {
      return Ok(Users::default());
    }

    let fields = fields.into().into_url_query();
    query.extend(fields);

    let vndb = Vndb::upgrade(&self.vndb)?;
    make_request!(vndb, get_json(Endpoint::User).query(query))
  }
}

impl Clone for Get {
  fn clone(&self) -> Self {
    Self { vndb: Weak::clone(&self.vndb) }
  }
}

#[bon::builder]
async fn get(
  #[builder(start_fn)] endpoint: Endpoint,
  semaphore: Weak<Semaphore>,
  query: Option<UrlQueryParams>,
  token: Option<&Token>,
  delay: Option<Duration>,
  timeout: Option<Duration>,
  user_agent: Option<&str>,
) -> Result<RawResponse> {
  request::<()>(endpoint)
    .method(Method::GET)
    .semaphore(semaphore)
    .maybe_query(query)
    .maybe_token(token)
    .maybe_delay(delay)
    .maybe_timeout(timeout)
    .maybe_user_agent(user_agent)
    .call()
    .await
}

#[bon::builder]
async fn get_json<Json>(
  #[builder(start_fn)] endpoint: Endpoint,
  semaphore: Weak<Semaphore>,
  query: Option<UrlQueryParams>,
  token: Option<&Token>,
  delay: Option<Duration>,
  timeout: Option<Duration>,
  user_agent: Option<&str>,
) -> Result<Json>
where
  Json: DeserializeOwned,
{
  request_json::<(), _>(endpoint)
    .method(Method::GET)
    .semaphore(semaphore)
    .maybe_query(query)
    .maybe_token(token)
    .maybe_delay(delay)
    .maybe_timeout(timeout)
    .maybe_user_agent(user_agent)
    .call()
    .await
}
