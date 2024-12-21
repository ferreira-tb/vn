use super::QueryField;
use crate::http::UrlQueryParams;
use crate::{
  impl_id_newtype, impl_id_newtype_from_numeric, impl_into_field_set, impl_string_set,
  impl_string_set_from_newtype, impl_string_set_from_numeric,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;
use strum::{Display, VariantArray};

#[cfg(feature = "regex")]
use {crate::impl_id_newtype_regex, regex::Regex, std::sync::LazyLock};

#[cfg(feature = "regex")]
static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^u\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct User {
  pub id: UserId,
  pub lengthvotes: Option<u32>,
  pub lengthvotes_sum: Option<u32>,
  pub username: String,
}

impl From<User> for UserId {
  fn from(u: User) -> Self {
    u.id
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserId(String);

impl UserId {
  pub const PREFIX: &'static str = "u";
}

impl_id_newtype!(UserId);
impl_id_newtype_from_numeric!(UserId::PREFIX, UserId);

#[cfg(feature = "regex")]
impl_id_newtype_regex!(UserId, ID_REGEX);

#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Users(HashMap<String, User>);

impl<'de> Deserialize<'de> for Users {
  fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    type Optional = HashMap<String, Option<User>>;
    Ok(Users(
      Optional::deserialize(deserializer)?
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect(),
    ))
  }
}

impl Users {
  pub fn into_inner(self) -> HashMap<String, User> {
    self.0
  }
}

impl Deref for Users {
  type Target = HashMap<String, User>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Users {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum UserField {
  #[serde(rename = "lengthvotes")]
  #[strum(serialize = "lengthvotes")]
  LengthVotes,

  #[serde(rename = "lengthvotes_sum")]
  #[strum(serialize = "lengthvotes_sum")]
  LengthVotesSum,
}

impl QueryField for UserField {}

impl_into_field_set!(UserField);

// We should not use `UserId` here, because the query is not restricted to user ids.
#[derive(Clone, Debug, Default)]
pub struct UserUrlQuery(HashSet<String>);

impl UserUrlQuery {
  pub const URL_QUERY_PARAM: &'static str = "q";

  pub fn into_query(self) -> UrlQueryParams {
    UrlQueryParams(
      self
        .0
        .into_iter()
        .map(|user| (Self::URL_QUERY_PARAM, user))
        .collect(),
    )
  }
}

impl_string_set!(UserUrlQuery);
impl_string_set_from_numeric!(UserId::PREFIX, UserUrlQuery);
impl_string_set_from_newtype!(UserUrlQuery, UserId);
