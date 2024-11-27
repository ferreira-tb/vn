use super::request_json;
use crate::error::Result;
use crate::http::{Endpoint, JsonQueryBuilder as Query, UrlQueryParams};
use crate::make_request;
use crate::model::prelude::*;
use crate::vndb::{Token, Vndb};
use http::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::Semaphore;

pub mod prelude {
  pub use super::{
    CharacterQuery, Post, ProducerQuery, ReleaseQuery, StaffQuery, TagQuery, TraitQuery,
    VisualNovelQuery,
  };
}

pub type CharacterQuery = Query<CharacterField, SortCharacterBy, Response<Character>>;
pub type ProducerQuery = Query<ProducerField, SortProducerBy, Response<Producer>>;
pub type ReleaseQuery = Query<ReleaseField, SortReleaseBy, Response<Release>>;
pub type StaffQuery = Query<StaffField, SortStaffBy, Response<Staff>>;
pub type TagQuery = Query<TagField, SortTagBy, Response<Tag>>;
pub type TraitQuery = Query<TraitField, SortTraitBy, Response<Trait>>;
pub type VisualNovelQuery = Query<VisualNovelField, SortVisualNovelBy, Response<VisualNovel>>;

pub struct Post {
  vndb: Weak<Vndb>,
}

macro_rules! post_request {
  ($self:expr, $kind:ident, $endpoint:ident) => {{
    let vndb = Weak::clone(&$self.vndb);
    $kind::new(Box::new(move |query| {
      Box::pin(async move {
        let vndb = Vndb::upgrade(&vndb)?;
        make_request!(vndb, post_json(Endpoint::$endpoint, &query))
      })
    }))
  }};
}

impl Post {
  pub fn new(vndb: Weak<Vndb>) -> Self {
    Self { vndb }
  }

  pub fn character(&self) -> CharacterQuery {
    post_request!(self, CharacterQuery, Character)
  }

  pub fn producer(&self) -> ProducerQuery {
    post_request!(self, ProducerQuery, Producer)
  }

  pub fn release(&self) -> ReleaseQuery {
    post_request!(self, ReleaseQuery, Release)
  }

  pub fn staff(&self) -> StaffQuery {
    post_request!(self, StaffQuery, Staff)
  }

  pub fn tag(&self) -> TagQuery {
    post_request!(self, TagQuery, Tag)
  }

  pub fn r#trait(&self) -> TraitQuery {
    post_request!(self, TraitQuery, Trait)
  }

  pub fn visual_novel(&self) -> VisualNovelQuery {
    post_request!(self, VisualNovelQuery, VisualNovel)
  }
}

impl Clone for Post {
  fn clone(&self) -> Self {
    Self { vndb: Weak::clone(&self.vndb) }
  }
}

#[bon::builder]
async fn post_json<Body, Json>(
  #[builder(start_fn)] endpoint: Endpoint,
  #[builder(start_fn)] body: &Body,
  query: Option<UrlQueryParams>,
  semaphore: &Semaphore,
  token: Option<&Token>,
  timeout: Option<Duration>,
  user_agent: Option<&str>,
) -> Result<Json>
where
  Body: Serialize + ?Sized,
  Json: DeserializeOwned,
{
  request_json(endpoint)
    .method(Method::POST)
    .semaphore(semaphore)
    .body(body)
    .maybe_query(query)
    .maybe_token(token)
    .maybe_timeout(timeout)
    .maybe_user_agent(user_agent)
    .call()
    .await
}
