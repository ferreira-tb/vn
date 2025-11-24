use super::schema::Language;
use super::{QueryField, SortQueryBy};
use crate::{
  impl_id_newtype,
  impl_id_newtype_from_numeric,
  impl_id_newtype_regex,
  impl_into_field_set,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use strum::{Display, EnumIs, VariantArray};

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^p\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Producer {
  pub aliases: Option<Vec<String>>,
  pub description: Option<String>,
  pub id: ProducerId,
  pub lang: Option<Language>,
  pub name: Option<String>,
  pub original: Option<String>,
  pub r#type: Option<ProducerType>,
}

impl From<Producer> for ProducerId {
  fn from(p: Producer) -> Self {
    p.id
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ProducerId(String);

impl ProducerId {
  pub const PREFIX: &'static str = "p";
}

impl_id_newtype!(ProducerId);
impl_id_newtype_regex!(ProducerId, ID_REGEX);
impl_id_newtype_from_numeric!(ProducerId::PREFIX, ProducerId);

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ProducerType {
  #[serde(rename = "ng")]
  #[strum(serialize = "ng")]
  AmateurGroup,

  #[serde(rename = "co")]
  #[strum(serialize = "co")]
  Company,

  #[serde(rename = "in")]
  #[strum(serialize = "in")]
  Individual,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ProducerField {
  #[serde(rename = "aliases")]
  #[strum(serialize = "aliases")]
  Aliases,

  #[serde(rename = "description")]
  #[strum(serialize = "description")]
  Description,

  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "lang")]
  #[strum(serialize = "lang")]
  Lang,

  #[serde(rename = "name")]
  #[strum(serialize = "name")]
  Name,

  #[serde(rename = "original")]
  #[strum(serialize = "original")]
  Original,

  #[serde(rename = "type")]
  #[strum(serialize = "type")]
  Type,
}

impl QueryField for ProducerField {}

impl_into_field_set!(ProducerField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SortProducerBy {
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

impl SortQueryBy for SortProducerBy {}
