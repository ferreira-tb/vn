use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype_from_numeric, impl_id_newtype_regex, impl_into_field_set};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::LazyLock;
use strum::{Display, EnumIs, VariantArray};

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^g\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Tag {
  pub aliases: Option<Vec<String>>,
  pub applicable: Option<bool>,
  pub category: Option<TagCategory>,
  pub description: Option<String>,
  pub id: TagId,
  pub name: Option<String>,
  pub searchable: Option<bool>,
  pub vn_count: Option<u32>,
}

impl From<Tag> for TagId {
  fn from(t: Tag) -> Self {
    t.id
  }
}

#[derive(
  Clone,
  Debug,
  Deserialize,
  Serialize,
  PartialEq,
  Eq,
  Hash,
  derive_more::Deref,
  derive_more::Display,
  derive_more::From,
  derive_more::Into,
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[from(&str, &String, String, Cow<'_, str>, Box<str>)]
pub struct TagId(String);

impl TagId {
  pub const PREFIX: &'static str = "g";
}

impl_id_newtype_regex!(TagId, ID_REGEX);
impl_id_newtype_from_numeric!(TagId::PREFIX, TagId);

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TagCategory {
  #[serde(rename = "cont")]
  #[strum(serialize = "cont")]
  Content,

  #[serde(rename = "ero")]
  #[strum(serialize = "ero")]
  Ero,

  #[serde(rename = "tech")]
  #[strum(serialize = "tech")]
  Technical,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TagField {
  #[serde(rename = "aliases")]
  #[strum(serialize = "aliases")]
  Aliases,

  #[serde(rename = "applicable")]
  #[strum(serialize = "applicable")]
  Applicable,

  #[serde(rename = "category")]
  #[strum(serialize = "category")]
  Category,

  #[serde(rename = "description")]
  #[strum(serialize = "description")]
  Description,

  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "name")]
  #[strum(serialize = "name")]
  Name,

  #[serde(rename = "searchable")]
  #[strum(serialize = "searchable")]
  Searchable,

  #[serde(rename = "vn_count")]
  #[strum(serialize = "vn_count")]
  VnCount,
}

impl QueryField for TagField {}

impl_into_field_set!(TagField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SortTagBy {
  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "name")]
  #[strum(serialize = "name")]
  Name,

  #[serde(rename = "searchrank")]
  #[strum(serialize = "searchrank")]
  SearchRank,

  #[serde(rename = "vn_count")]
  #[strum(serialize = "vn_count")]
  VnCount,
}

impl SortQueryBy for SortTagBy {}
