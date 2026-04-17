use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype, impl_into_field_set};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock};
use strum::{Display, EnumString, VariantArray};

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
  pub sexual: Option<bool>,
}

impl From<Trait> for TraitId {
  fn from(t: Trait) -> Self {
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
pub struct TraitId(Arc<str>);

impl TraitId {
  pub const PREFIX: &'static str = "i";
}

impl_id_newtype!(TraitId, ID_REGEX);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, EnumString, VariantArray)]
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

  #[serde(rename = "sexual")]
  #[strum(serialize = "sexual")]
  Sexual,
}

impl QueryField for TraitField {}

impl_into_field_set!(TraitField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, EnumString)]
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
