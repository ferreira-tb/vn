use super::FieldSet;
use crate::model::QueryField;

#[derive(Clone, Debug, Default)]
pub struct UrlQueryParams(pub(crate) Vec<(&'static str, String)>);

impl UrlQueryParams {
  pub fn extend(&mut self, other: Self) {
    self.0.extend(other.0);
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl<T: QueryField> From<FieldSet<T>> for UrlQueryParams {
  fn from(set: FieldSet<T>) -> Self {
    set.into_url_query()
  }
}
