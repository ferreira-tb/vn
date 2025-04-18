mod json;
mod url;

use crate::model::QueryField;
use itertools::Itertools;
pub use json::{JsonQuery, JsonQueryBuilder, JsonQueryFilter};
use std::collections::HashSet;
use std::marker::PhantomData;
pub use url::UrlQueryParams;

#[derive(Clone, Debug)]
pub struct FieldSet<T: QueryField> {
  inner: HashSet<String>,
  marker: PhantomData<T>,
}

impl<T: QueryField> FieldSet<T> {
  pub const URL_QUERY_PARAM: &'static str = "fields";

  pub fn from_raw<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = String>,
  {
    Self {
      inner: iter.into_iter().collect(),
      marker: PhantomData,
    }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      inner: HashSet::with_capacity(capacity),
      marker: PhantomData,
    }
  }

  pub fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = T>,
  {
    self
      .inner
      .extend(iter.into_iter().map(|field| field.to_string()));
  }

  pub fn extend_raw<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = String>,
  {
    self.inner.extend(iter);
  }

  pub fn insert(&mut self, field: &T) {
    self.inner.insert(field.to_string());
  }

  pub fn insert_raw(&mut self, field: &str) {
    self.inner.insert(field.to_string());
  }

  pub fn into_url_query(self) -> UrlQueryParams {
    UrlQueryParams(vec![(Self::URL_QUERY_PARAM, self.join())])
  }

  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  pub fn join(self) -> String {
    self.inner.iter().join(",")
  }

  pub fn remove(&mut self, field: &T) -> bool {
    self.inner.remove(&field.to_string())
  }

  pub fn reserve(&mut self, additional: usize) {
    self.inner.reserve(additional);
  }

  pub fn shrink_to_fit(&mut self) {
    self.inner.shrink_to_fit();
  }
}

impl<T: QueryField> Default for FieldSet<T> {
  fn default() -> Self {
    Self {
      inner: HashSet::new(),
      marker: PhantomData,
    }
  }
}
