use super::schema::Language;
use super::{QueryField, SortQueryBy};
use crate::model::release::ExternalLink;
use crate::{impl_id_newtype, impl_into_field_set};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock};
use strum::{Display, EnumIs, EnumString, VariantArray};

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^p\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Producer {
  pub aliases: Option<Vec<String>>,
  pub description: Option<String>,
  pub extlinks: Option<Vec<ExternalLink>>,
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
pub struct ProducerId(#[cfg_attr(feature = "specta", specta(type = String))] Arc<str>);

impl ProducerId {
  pub const PREFIX: char = 'p';
}

impl_id_newtype!(Producer, ProducerId, ID_REGEX);

#[non_exhaustive]
#[remain::sorted]
#[derive(
  Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs, EnumString,
)]
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
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, EnumString, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ProducerField {
  #[serde(rename = "aliases")]
  #[strum(serialize = "aliases")]
  Aliases,

  #[serde(rename = "description")]
  #[strum(serialize = "description")]
  Description,

  #[serde(rename = "extlinks.id")]
  #[strum(serialize = "extlinks.id")]
  ExtlinksId,

  #[serde(rename = "extlinks.label")]
  #[strum(serialize = "extlinks.label")]
  ExtlinksLabel,

  #[serde(rename = "extlinks.name")]
  #[strum(serialize = "extlinks.name")]
  ExtlinksName,

  #[serde(rename = "extlinks.url")]
  #[strum(serialize = "extlinks.url")]
  ExtlinksUrl,

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
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, EnumString)]
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
