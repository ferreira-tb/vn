use super::FieldSet;
use crate::error::Result;
use crate::model::user::UserId;
use crate::model::{QueryField, SortQueryBy};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use std::num::NonZeroU16;

type RequestFn<T> = Box<dyn FnOnce(JsonQuery) -> BoxFuture<'static, Result<T>> + Send>;

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonQuery {
  compact_filters: bool,
  count: bool,
  fields: Option<String>,
  filters: JsonValue,
  normalized_filters: bool,
  page: Option<NonZeroU16>,
  results: Option<u8>,
  reverse: bool,
  sort: Option<String>,
  user: Option<UserId>,
}

#[remain::sorted]
pub struct JsonQueryBuilder<Field, Sort, Value>
where
  Field: QueryField,
  Sort: SortQueryBy,
  Value: Serialize,
{
  compact_filters: bool,
  count: bool,
  fields: Option<FieldSet<Field>>,
  filters: JsonQueryFilter,
  normalized_filters: bool,
  page: Option<NonZeroU16>,
  results: Option<u8>,
  reverse: bool,
  send_request: RequestFn<Value>,
  sort: Option<Sort>,
  user: Option<UserId>,
}

impl<Field, Sort, Value> JsonQueryBuilder<Field, Sort, Value>
where
  Field: QueryField,
  Sort: SortQueryBy,
  Value: Serialize,
{
  pub(in crate::http) fn new(send_request: RequestFn<Value>) -> Self {
    Self {
      compact_filters: false,
      count: false,
      fields: None,
      filters: JsonQueryFilter::default(),
      normalized_filters: false,
      page: None,
      results: None,
      reverse: false,
      send_request,
      sort: None,
      user: None,
    }
  }

  #[must_use]
  pub fn compact_filters(mut self) -> Self {
    self.compact_filters = true;
    self
  }

  #[must_use]
  pub fn count(mut self) -> Self {
    self.count = true;
    self
  }

  #[must_use]
  pub fn fields(mut self, fields: impl Into<FieldSet<Field>>) -> Self {
    let fields: FieldSet<Field> = fields.into();
    if let Some(set) = &mut self.fields {
      set.inner.extend(fields.inner);
    } else {
      self.fields = Some(fields);
    }

    self
  }

  #[must_use]
  pub fn filters(mut self, filter: JsonQueryFilter) -> Self {
    self.filters = filter;
    self
  }

  #[must_use]
  pub fn normalized_filters(mut self) -> Self {
    self.normalized_filters = true;
    self
  }

  #[must_use]
  pub fn page(mut self, page: u16) -> Self {
    self.page = Some(page.try_into().unwrap_or_else(|_| {
      // SAFETY: Safe as long as the value is not zero.
      unsafe { NonZeroU16::new_unchecked(1) }
    }));

    self
  }

  #[must_use]
  pub fn raw_fields<I>(mut self, fields: I) -> Self
  where
    I: IntoIterator<Item = String>,
  {
    let set = FieldSet::from_raw(fields);
    if let Some(current) = &mut self.fields {
      current.inner.extend(set.inner);
    } else {
      self.fields = Some(set);
    }

    self
  }

  #[must_use]
  pub fn results(mut self, results: u8) -> Self {
    self.results = Some(results.min(100));
    self
  }

  #[must_use]
  pub fn reverse(mut self) -> Self {
    self.reverse = true;
    self
  }

  #[must_use]
  pub fn sort(mut self, sort: Sort) -> Self {
    self.sort = Some(sort);
    self
  }

  #[must_use]
  pub fn user(mut self, user: impl Into<UserId>) -> Self {
    self.user = Some(user.into());
    self
  }

  pub async fn send(self) -> Result<Value> {
    let query = JsonQuery {
      compact_filters: self.compact_filters,
      count: self.count,
      fields: self.fields.map(FieldSet::join),
      filters: self.filters.into_inner(),
      normalized_filters: self.normalized_filters,
      page: self.page,
      results: self.results,
      reverse: self.reverse,
      sort: self.sort.map(|s| s.to_string()),
      user: self.user,
    };

    (self.send_request)(query).await
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonQueryFilter(JsonValue);

impl JsonQueryFilter {
  pub fn new(value: JsonValue) -> Self {
    Self(value)
  }

  pub fn clear(&mut self) {
    self.0 = JsonValue::Null;
  }

  pub fn into_inner(self) -> JsonValue {
    self.0
  }
}

impl Default for JsonQueryFilter {
  fn default() -> Self {
    Self(JsonValue::Null)
  }
}

impl From<JsonValue> for JsonQueryFilter {
  fn from(value: JsonValue) -> Self {
    Self(value)
  }
}

impl TryFrom<&str> for JsonQueryFilter {
  type Error = crate::error::Error;

  fn try_from(value: &str) -> Result<Self> {
    serde_json::from_str(value)
      .map(Self)
      .map_err(Into::into)
  }
}

impl TryFrom<String> for JsonQueryFilter {
  type Error = crate::error::Error;

  fn try_from(value: String) -> Result<Self> {
    Self::try_from(value.as_str())
  }
}

impl TryFrom<&String> for JsonQueryFilter {
  type Error = crate::error::Error;

  fn try_from(value: &String) -> Result<Self> {
    Self::try_from(value.as_str())
  }
}

impl TryFrom<Cow<'_, str>> for JsonQueryFilter {
  type Error = crate::error::Error;

  fn try_from(value: Cow<'_, str>) -> Result<Self> {
    Self::try_from(value.as_ref())
  }
}
