use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype, impl_into_field_set};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock};
use strum::{Display, EnumIs, EnumString, VariantArray};

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
  Debug,
  Deserialize,
  Serialize,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  derive_more::Display,
  derive_more::Into,
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TagId(#[cfg_attr(feature = "specta", specta(type = String))] Arc<str>);

impl TagId {
  pub const PREFIX: char = 'g';
}

impl_id_newtype!(Tag, TagId, ID_REGEX);

#[non_exhaustive]
#[remain::sorted]
#[derive(
  Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs, EnumString,
)]
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
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, EnumString, VariantArray)]
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
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, EnumString)]
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
