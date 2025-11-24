use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype, impl_id_newtype_from_numeric, impl_into_field_set};
use serde::{Deserialize, Serialize};
use strum::{Display, VariantArray};

#[cfg(feature = "regex")]
use {crate::impl_id_newtype_regex, regex::Regex, std::sync::LazyLock};

#[cfg(feature = "regex")]
static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^i\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Trait {
  pub aliases: Option<Vec<String>>,
  pub applicable: Option<bool>,
  pub char_count: Option<u32>,
  pub description: Option<String>,
  pub group_id: Option<TraitId>,
  pub group_name: Option<String>,
  pub id: TraitId,
  pub name: Option<String>,
  pub searchable: Option<bool>,
}

impl From<Trait> for TraitId {
  fn from(t: Trait) -> Self {
    t.id
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TraitId(String);

impl TraitId {
  pub const PREFIX: &'static str = "i";
}

impl_id_newtype!(TraitId);
impl_id_newtype_from_numeric!(TraitId::PREFIX, TraitId);

#[cfg(feature = "regex")]
impl_id_newtype_regex!(TraitId, ID_REGEX);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TraitField {
  #[serde(rename = "aliases")]
  #[strum(serialize = "aliases")]
  Aliases,

  #[serde(rename = "applicable")]
  #[strum(serialize = "applicable")]
  Applicable,

  #[serde(rename = "char_count")]
  #[strum(serialize = "char_count")]
  CharCount,

  #[serde(rename = "description")]
  #[strum(serialize = "description")]
  Description,

  #[serde(rename = "group_id")]
  #[strum(serialize = "group_id")]
  GroupId,

  #[serde(rename = "group_name")]
  #[strum(serialize = "group_name")]
  GroupName,

  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "name")]
  #[strum(serialize = "name")]
  Name,

  #[serde(rename = "searchable")]
  #[strum(serialize = "searchable")]
  Searchable,
}

impl QueryField for TraitField {}

impl_into_field_set!(TraitField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SortTraitBy {
  #[serde(rename = "char_count")]
  #[strum(serialize = "char_count")]
  CharCount,

  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "name")]
  #[strum(serialize = "name")]
  Name,

  #[serde(rename = "searchrank")]
  #[strum(serialize = "searchrank")]
  SearchRank,
}

impl SortQueryBy for SortTraitBy {}
