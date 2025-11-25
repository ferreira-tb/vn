use super::release::ExternalLink;
use super::schema::Language;
use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype_from_numeric, impl_id_newtype_regex, impl_into_field_set};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::LazyLock;
use strum::{Display, EnumIs, VariantArray};

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^s\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Staff {
  pub aid: Option<u32>,
  pub aliases: Option<Vec<StaffAlias>>,
  pub description: Option<String>,
  pub extlinks: Option<Vec<ExternalLink>>,
  pub gender: Option<StaffGender>,
  pub id: StaffId,
  pub ismain: Option<bool>,
  pub lang: Option<Language>,
  pub name: Option<String>,
  pub original: Option<String>,
}

impl From<Staff> for StaffId {
  fn from(s: Staff) -> Self {
    s.id
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
pub struct StaffId(String);

impl StaffId {
  pub const PREFIX: &'static str = "s";
}

impl_id_newtype_regex!(StaffId, ID_REGEX);
impl_id_newtype_from_numeric!(StaffId::PREFIX, StaffId);

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StaffAlias {
  pub aid: Option<u32>,
  pub ismain: Option<bool>,
  pub latin: Option<String>,
  pub name: Option<String>,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum StaffGender {
  #[serde(rename = "f")]
  #[strum(serialize = "f")]
  Female,

  #[serde(rename = "m")]
  #[strum(serialize = "m")]
  Male,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum StaffField {
  #[serde(rename = "aid")]
  #[strum(serialize = "aid")]
  Aid,

  #[serde(rename = "aliases.aid")]
  #[strum(serialize = "aliases.aid")]
  AliasesAid,

  #[serde(rename = "aliases.ismain")]
  #[strum(serialize = "aliases.ismain")]
  AliasesIsMain,

  #[serde(rename = "aliases.latin")]
  #[strum(serialize = "aliases.latin")]
  AliasesLatin,

  #[serde(rename = "aliases.name")]
  #[strum(serialize = "aliases.name")]
  AliasesName,

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

  #[serde(rename = "gender")]
  #[strum(serialize = "gender")]
  Gender,

  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "ismain")]
  #[strum(serialize = "ismain")]
  IsMain,

  #[serde(rename = "lang")]
  #[strum(serialize = "lang")]
  Lang,

  #[serde(rename = "name")]
  #[strum(serialize = "name")]
  Name,

  #[serde(rename = "original")]
  #[strum(serialize = "original")]
  Original,
}

impl QueryField for StaffField {}

impl_into_field_set!(StaffField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SortStaffBy {
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

impl SortQueryBy for SortStaffBy {}
