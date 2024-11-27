use super::producer::Producer;
use super::schema::Language;
use super::visual_novel::{VisualNovel, VisualNovelId, VisualNovelImage};
use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype, impl_id_newtype_from_numeric, impl_into_field_set};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value as JsonValue;
use strum::{Display, EnumIs, VariantArray};

#[cfg(feature = "regex")]
use {crate::impl_id_newtype_regex, regex::Regex, std::sync::LazyLock};

#[cfg(feature = "regex")]
static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^r\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Release {
  pub alttitle: Option<String>,
  pub catalog: Option<String>,
  pub engine: Option<String>,
  pub extlinks: Option<Vec<ExternalLink>>,
  pub freeware: Option<bool>,
  pub gtin: Option<String>,
  pub has_ero: Option<bool>,
  pub id: ReleaseId,
  pub languages: Option<Vec<ReleaseLanguage>>,
  pub media: Option<Vec<ReleaseMedia>>,
  pub minage: Option<u32>,
  pub notes: Option<String>,
  pub official: Option<bool>,
  pub patch: Option<bool>,
  pub platforms: Option<Vec<String>>,
  pub released: Option<String>,
  pub resolution: Option<ReleaseResolution>,
  pub title: Option<String>,
  pub uncensored: Option<bool>,
  pub voiced: Option<ReleaseVoiced>,
}

impl From<Release> for ReleaseId {
  fn from(r: Release) -> Self {
    r.id
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReleaseId(String);

impl ReleaseId {
  pub const PREFIX: &'static str = "r";
}

impl_id_newtype!(ReleaseId);
impl_id_newtype_from_numeric!(ReleaseId::PREFIX, ReleaseId);

#[cfg(feature = "regex")]
impl_id_newtype_regex!(ReleaseId, ID_REGEX);

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReleaseImage {
  #[serde(flatten)]
  pub image: VisualNovelImage,
  pub languages: Option<Vec<Language>>,
  pub photo: Option<bool>,
  pub r#type: Option<ReleaseImageType>,
  pub vn: Option<VisualNovelId>,
}

impl From<ReleaseImage> for VisualNovelImage {
  fn from(ri: ReleaseImage) -> Self {
    ri.image
  }
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ReleaseImageType {
  Dig,
  PkgBack,
  PkgContent,
  PkgFront,
  PkgMed,
  PkgSide,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReleaseLanguage {
  pub lang: Option<Language>,
  pub latin: Option<String>,
  pub main: Option<bool>,
  pub mtl: Option<bool>,
  pub title: Option<String>,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReleaseMedia {
  pub medium: Option<String>,
  pub qty: Option<u32>,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReleaseProducer {
  pub developer: Option<bool>,
  #[serde(flatten)]
  pub producer: Producer,
  pub publisher: Option<bool>,
}

impl From<ReleaseProducer> for Producer {
  fn from(rp: ReleaseProducer) -> Self {
    rp.producer
  }
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(untagged)]
pub enum ReleaseResolution {
  NonStandard(String),
  Standard([u32; 2]),
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ReleaseType {
  #[serde(rename = "complete")]
  #[strum(serialize = "complete")]
  Complete,

  #[serde(rename = "partial")]
  #[strum(serialize = "partial")]
  Partial,

  #[serde(rename = "trial")]
  #[strum(serialize = "trial")]
  Trial,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReleaseVisualNovel {
  pub rtype: Option<ReleaseType>,
  #[serde(flatten)]
  pub visual_novel: VisualNovel,
}

impl From<ReleaseVisualNovel> for VisualNovel {
  fn from(rvn: ReleaseVisualNovel) -> Self {
    rvn.visual_novel
  }
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ReleaseVoiced {
  FullyVoiced,
  NotVoiced,
  OnlyEroScenes,
  PartiallyVoiced,
}

impl<'de> Deserialize<'de> for ReleaseVoiced {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    match u8::deserialize(deserializer)? {
      1 => Ok(ReleaseVoiced::NotVoiced),
      2 => Ok(ReleaseVoiced::OnlyEroScenes),
      3 => Ok(ReleaseVoiced::PartiallyVoiced),
      4 => Ok(ReleaseVoiced::FullyVoiced),
      _ => Err(D::Error::custom("invalid release voiced value")),
    }
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ExternalLink {
  pub id: Option<JsonValue>,
  pub label: Option<String>,
  pub name: Option<String>,
  pub url: Option<String>,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ReleaseField {
  #[serde(rename = "alttitle")]
  #[strum(serialize = "alttitle")]
  AltTitle,

  #[serde(rename = "catalog")]
  #[strum(serialize = "catalog")]
  Catalog,

  #[serde(rename = "engine")]
  #[strum(serialize = "engine")]
  Engine,

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

  #[serde(rename = "freeware")]
  #[strum(serialize = "freeware")]
  Freeware,

  #[serde(rename = "gtin")]
  #[strum(serialize = "gtin")]
  Gtin,

  #[serde(rename = "has_ero")]
  #[strum(serialize = "has_ero")]
  HasEro,

  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "images.id")]
  #[strum(serialize = "images.id")]
  ImagesId,

  #[serde(rename = "images.languages")]
  #[strum(serialize = "images.languages")]
  ImagesLanguages,

  #[serde(rename = "images.photo")]
  #[strum(serialize = "images.photo")]
  ImagesPhoto,

  #[serde(rename = "images.type")]
  #[strum(serialize = "images.type")]
  ImagesType,

  #[serde(rename = "images.url")]
  #[strum(serialize = "images.url")]
  ImagesUrl,

  #[serde(rename = "images.vn")]
  #[strum(serialize = "images.vn")]
  ImagesVn,

  #[serde(rename = "languages.lang")]
  #[strum(serialize = "languages.lang")]
  LanguagesLang,

  #[serde(rename = "languages.latin")]
  #[strum(serialize = "languages.latin")]
  LanguagesLatin,

  #[serde(rename = "languages.main")]
  #[strum(serialize = "languages.main")]
  LanguagesMain,

  #[serde(rename = "languages.mtl")]
  #[strum(serialize = "languages.mtl")]
  LanguagesMtl,

  #[serde(rename = "languages.title")]
  #[strum(serialize = "languages.title")]
  LanguagesTitle,

  #[serde(rename = "media.medium")]
  #[strum(serialize = "media.medium")]
  MediaMedium,

  #[serde(rename = "media.qty")]
  #[strum(serialize = "media.qty")]
  MediaQty,

  #[serde(rename = "minage")]
  #[strum(serialize = "minage")]
  MinAge,

  #[serde(rename = "notes")]
  #[strum(serialize = "notes")]
  Notes,

  #[serde(rename = "official")]
  #[strum(serialize = "official")]
  Official,

  #[serde(rename = "patch")]
  #[strum(serialize = "patch")]
  Patch,

  #[serde(rename = "platforms")]
  #[strum(serialize = "platforms")]
  Platforms,

  #[serde(rename = "producers.developer")]
  #[strum(serialize = "producers.developer")]
  ProducersDeveloper,

  #[serde(rename = "producers.id")]
  #[strum(serialize = "producers.id")]
  ProducersId,

  #[serde(rename = "producers.publisher")]
  #[strum(serialize = "producers.publisher")]
  ProducersPublisher,

  #[serde(rename = "released")]
  #[strum(serialize = "released")]
  Released,

  #[serde(rename = "resolution")]
  #[strum(serialize = "resolution")]
  Resolution,

  #[serde(rename = "title")]
  #[strum(serialize = "title")]
  Title,

  #[serde(rename = "uncensored")]
  #[strum(serialize = "uncensored")]
  Uncensored,

  #[serde(rename = "vns.id")]
  #[strum(serialize = "vns.id")]
  VisualNovelId,

  #[serde(rename = "vns.rtype")]
  #[strum(serialize = "vns.rtype")]
  VisualNovelRType,

  #[serde(rename = "voiced")]
  #[strum(serialize = "voiced")]
  Voiced,
}

impl QueryField for ReleaseField {}

impl_into_field_set!(ReleaseField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SortReleaseBy {
  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "released")]
  #[strum(serialize = "released")]
  Released,

  #[serde(rename = "searchrank")]
  #[strum(serialize = "searchrank")]
  SearchRank,

  #[serde(rename = "title")]
  #[strum(serialize = "title")]
  Title,
}

impl SortQueryBy for SortReleaseBy {}
