use crate::error::{Error, Result};
use crate::http::request::get::Get;
use crate::http::request::post::prelude::*;
use crate::model::character::CharacterId;
use crate::model::producer::ProducerId;
use crate::model::release::ReleaseId;
use crate::model::staff::StaffId;
use crate::model::tag::TagId;
use crate::model::r#trait::TraitId;
use crate::model::user::{User, UserField, UserId};
use crate::model::visual_novel::VisualNovelId;
use std::num::NonZeroU8;
use std::ops::Deref;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::sync::Semaphore;

const CONCURRENCY: NonZeroU8 = NonZeroU8::new(10).unwrap();

#[derive(Debug)]
pub struct Vndb {
  pub(crate) semaphore: Arc<Semaphore>,
  pub(crate) token: Option<Token>,
  pub(crate) delay: Option<Duration>,
  pub(crate) timeout: Option<Duration>,
  pub(crate) user_agent: Option<String>,
}

impl Vndb {
  pub fn new() -> Arc<Self> {
    let concurrent_requests = usize::from(CONCURRENCY.get());
    let semaphore = Semaphore::new(concurrent_requests);
    Arc::new(Self {
      semaphore: Arc::new(semaphore),
      token: None,
      delay: None,
      timeout: None,
      user_agent: None,
    })
  }

  pub fn builder() -> VndbBuilder {
    VndbBuilder::new()
  }

  pub fn with_token(token: impl Into<Token>) -> Arc<Self> {
    Self::builder().token(token).build()
  }

  pub fn get(self: &Arc<Self>) -> Get {
    Get::new(Arc::downgrade(self))
  }

  pub fn post(self: &Arc<Self>) -> Post {
    Post::new(Arc::downgrade(self))
  }

  pub(crate) fn upgrade(weak: &Weak<Self>) -> Result<Arc<Self>> {
    weak.upgrade().ok_or(Error::Disconnected)
  }
}

macro_rules! find {
  ($vndb:expr, $id:expr, $post_fn:ident, $field:ident) => {{
    let filters = serde_json::json!(["id", "=", $id]);
    $vndb
      .post()
      .$post_fn()
      .filters(filters.into())
  }};
}

macro_rules! search {
  ($vndb:expr, $query:expr, $post_fn:ident, $field:ident) => {{
    let query = $query.as_ref();
    let filters = serde_json::json!(["search", "=", query]);
    $vndb
      .post()
      .$post_fn()
      .filters(filters.into())
  }};
}

impl Vndb {
  pub fn find_character<Id>(self: &Arc<Self>, id: Id) -> CharacterQuery
  where
    Id: Into<CharacterId>,
  {
    let id: CharacterId = id.into();
    find!(self, id, character, CharacterField)
  }

  pub fn find_producer<Id>(self: &Arc<Self>, id: Id) -> ProducerQuery
  where
    Id: Into<ProducerId>,
  {
    let id: ProducerId = id.into();
    find!(self, id, producer, ProducerField)
  }

  pub fn find_release<Id>(self: &Arc<Self>, id: Id) -> ReleaseQuery
  where
    Id: Into<ReleaseId>,
  {
    let id: ReleaseId = id.into();
    find!(self, id, release, ReleaseField)
  }

  pub fn find_staff<Id>(self: &Arc<Self>, id: Id) -> StaffQuery
  where
    Id: Into<StaffId>,
  {
    let id: StaffId = id.into();
    find!(self, id, staff, StaffField)
  }

  pub fn find_tag<Id>(self: &Arc<Self>, id: Id) -> TagQuery
  where
    Id: Into<TagId>,
  {
    let id: TagId = id.into();
    find!(self, id, tag, TagField)
  }

  pub fn find_trait<Id>(self: &Arc<Self>, id: Id) -> TraitQuery
  where
    Id: Into<TraitId>,
  {
    let id: TraitId = id.into();
    find!(self, id, r#trait, TraitField)
  }

  pub async fn find_user<Id>(self: &Arc<Self>, id: Id) -> Result<Option<User>>
  where
    Id: Into<UserId>,
  {
    let id: UserId = id.into();
    let user = self
      .get()
      .user(id, UserField::all())
      .await?
      .into_inner()
      .into_values()
      .next();

    Ok(user)
  }

  pub fn find_visual_novel<Id>(self: &Arc<Self>, id: Id) -> VisualNovelQuery
  where
    Id: Into<VisualNovelId>,
  {
    let id: VisualNovelId = id.into();
    find!(self, id, visual_novel, VisualNovelField)
  }

  pub fn search_character(self: &Arc<Self>, query: impl AsRef<str>) -> CharacterQuery {
    search!(self, query, character, CharacterField)
  }

  pub fn search_producer(self: &Arc<Self>, query: impl AsRef<str>) -> ProducerQuery {
    search!(self, query, producer, ProducerField)
  }

  pub fn search_release(self: &Arc<Self>, query: impl AsRef<str>) -> ReleaseQuery {
    search!(self, query, release, ReleaseField)
  }

  pub fn search_staff(self: &Arc<Self>, query: impl AsRef<str>) -> StaffQuery {
    search!(self, query, staff, StaffField)
  }

  pub fn search_tag(self: &Arc<Self>, query: impl AsRef<str>) -> TagQuery {
    search!(self, query, tag, TagField)
  }

  pub fn search_trait(self: &Arc<Self>, query: impl AsRef<str>) -> TraitQuery {
    search!(self, query, r#trait, TraitField)
  }

  pub fn search_visual_novel(self: &Arc<Self>, query: impl AsRef<str>) -> VisualNovelQuery {
    search!(self, query, visual_novel, VisualNovelField)
  }
}

#[derive(Debug)]
pub struct VndbBuilder {
  max_concurrent_requests: NonZeroU8,
  token: Option<Token>,
  delay: Option<Duration>,
  timeout: Option<Duration>,
  user_agent: Option<String>,
}

impl VndbBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  #[must_use]
  pub fn max_concurrent_requests(mut self, amount: u8) -> Self {
    self.max_concurrent_requests = NonZeroU8::new(amount).unwrap_or(CONCURRENCY);
    self
  }

  #[must_use]
  pub fn token(mut self, token: impl Into<Token>) -> Self {
    self.token = Some(token.into());
    self
  }

  #[must_use]
  pub fn delay(mut self, delay: Duration) -> Self {
    self.delay = Some(delay);
    self
  }

  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.timeout = Some(timeout);
    self
  }

  #[must_use]
  pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
    self.user_agent = Some(user_agent.into());
    self
  }

  pub fn build(self) -> Arc<Vndb> {
    let max_concurrent_requests = self.max_concurrent_requests.get();
    let semaphore = Semaphore::new(usize::from(max_concurrent_requests));
    let vndb = Vndb {
      semaphore: Arc::new(semaphore),
      token: self.token,
      delay: self.delay,
      timeout: self.timeout,
      user_agent: self.user_agent,
    };

    Arc::new(vndb)
  }
}

impl Default for VndbBuilder {
  fn default() -> Self {
    Self {
      max_concurrent_requests: CONCURRENCY,
      token: None,
      delay: None,
      timeout: None,
      user_agent: None,
    }
  }
}

/// See: <https://api.vndb.org/kana#user-authentication>
#[derive(Clone, Debug)]
pub struct Token(Box<str>);

impl Token {
  pub(crate) fn to_header(&self) -> String {
    format!("Token {}", self.0)
  }
}

impl<T: AsRef<str>> From<T> for Token {
  fn from(token: T) -> Self {
    Self(Box::from(token.as_ref()))
  }
}

impl Deref for Token {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
