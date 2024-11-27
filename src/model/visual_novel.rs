use super::producer::Producer;
use super::release::ExternalLink;
use super::schema::Language;
use super::staff::Staff;
use super::tag::Tag;
use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype, impl_id_newtype_from_numeric, impl_into_field_set};
use serde::{Deserialize, Deserializer, Serialize};
use std::result::Result as StdResult;
use strum::{Display, EnumIs, VariantArray};

#[cfg(feature = "regex")]
use {crate::impl_id_newtype_regex, regex::Regex, std::sync::LazyLock};

#[cfg(feature = "regex")]
static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^v\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovel {
  pub aliases: Option<Vec<String>>,
  pub alttitle: Option<String>,
  pub average: Option<f32>,
  pub description: Option<String>,
  pub developers: Option<Vec<VisualNovelDeveloper>>,
  pub devstatus: Option<VisualNovelDevStatus>,
  pub editions: Option<Vec<VisualNovelEdition>>,
  pub extlinks: Option<Vec<ExternalLink>>,
  pub id: VisualNovelId,
  pub image: Option<VisualNovelImage>,
  pub languages: Option<Vec<Language>>,
  pub length: Option<VisualNovelLength>,
  pub length_minutes: Option<u32>,
  pub length_votes: Option<u32>,
  pub olang: Option<Language>,
  pub platforms: Option<Vec<String>>,
  pub rating: Option<f32>,
  pub relations: Option<Vec<VisualNovelRelation>>,
  pub released: Option<String>,
  pub screenshots: Option<Vec<VisualNovelScreenShot>>,
  pub staff: Option<Vec<VisualNovelStaff>>,
  pub tags: Option<Vec<VisualNovelTag>>,
  pub title: Option<String>,
  pub titles: Option<Vec<VisualNovelTitle>>,
  pub va: Option<Vec<VisualNovelVoiceActor>>,
  pub votecount: Option<u32>,
}

impl From<VisualNovel> for VisualNovelId {
  fn from(v: VisualNovel) -> Self {
    v.id
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelId(String);

impl VisualNovelId {
  pub const PREFIX: &'static str = "v";
}

impl_id_newtype!(VisualNovelId);
impl_id_newtype_from_numeric!(VisualNovelId::PREFIX, VisualNovelId);

#[cfg(feature = "regex")]
impl_id_newtype_regex!(VisualNovelId, ID_REGEX);

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum VisualNovelDevStatus {
  Cancelled,
  Finished,
  InDevelopment,
}

impl<'de> Deserialize<'de> for VisualNovelDevStatus {
  fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    match u8::deserialize(deserializer)? {
      0 => Ok(Self::Finished),
      1 => Ok(Self::InDevelopment),
      2 => Ok(Self::Cancelled),
      _ => Err(Error::custom("invalid devstatus")),
    }
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelDeveloper {
  #[serde(flatten)]
  pub producer: Producer,
}

impl From<VisualNovelDeveloper> for Producer {
  fn from(v: VisualNovelDeveloper) -> Self {
    v.producer
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelEdition {
  pub eid: Option<u32>,
  pub lang: Option<Language>,
  pub name: Option<String>,
  pub official: Option<bool>,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelImage {
  pub dims: Option<[u32; 2]>,
  pub id: Option<String>,
  pub sexual: Option<f32>,
  pub thumbnail: Option<String>,
  pub thumbnail_dims: Option<[u32; 2]>,
  pub url: Option<String>,
  pub violence: Option<f32>,
  pub votecount: Option<u32>,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum VisualNovelLength {
  VeryShort,
  Short,
  Average,
  Long,
  VeryLong,
}

impl<'de> Deserialize<'de> for VisualNovelLength {
  fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    match u8::deserialize(deserializer)? {
      1 => Ok(Self::VeryShort),
      2 => Ok(Self::Short),
      3 => Ok(Self::Average),
      4 => Ok(Self::Long),
      5 => Ok(Self::VeryLong),
      _ => Err(D::Error::custom("invalid visual novel length")),
    }
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelRelation {
  pub relation: Option<String>,
  pub relation_official: Option<bool>,
  #[serde(flatten)]
  pub visual_novel: VisualNovel,
}

impl From<VisualNovelRelation> for VisualNovel {
  fn from(v: VisualNovelRelation) -> Self {
    v.visual_novel
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelScreenShot {
  #[serde(flatten)]
  pub image: VisualNovelImage,
}

impl From<VisualNovelScreenShot> for VisualNovelImage {
  fn from(v: VisualNovelScreenShot) -> Self {
    v.image
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelStaff {
  pub eid: Option<u32>,
  pub note: Option<String>,
  pub role: Option<String>,
  #[serde(flatten)]
  pub staff: Staff,
}

impl From<VisualNovelStaff> for Staff {
  fn from(v: VisualNovelStaff) -> Self {
    v.staff
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelTag {
  pub lie: Option<bool>,
  pub rating: Option<f32>,
  pub spoiler: Option<u8>,
  #[serde(flatten)]
  pub tag: Tag,
}

impl From<VisualNovelTag> for Tag {
  fn from(v: VisualNovelTag) -> Self {
    v.tag
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelTitle {
  pub lang: Option<Language>,
  pub latin: Option<String>,
  pub main: Option<bool>,
  pub official: Option<bool>,
  pub title: Option<String>,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VisualNovelVoiceActor {
  // pub character: Option<Character>,
  pub note: Option<String>,
  pub staff: Option<Staff>,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum VisualNovelField {
  #[serde(rename = "aliases")]
  #[strum(serialize = "aliases")]
  Aliases,

  #[serde(rename = "alttitle")]
  #[strum(serialize = "alttitle")]
  AltTitle,

  #[serde(rename = "average")]
  #[strum(serialize = "average")]
  Average,

  #[serde(rename = "description")]
  #[strum(serialize = "description")]
  Description,

  #[serde(rename = "developers.aliases")]
  #[strum(serialize = "developers.aliases")]
  DevelopersAliases,

  #[serde(rename = "developers.description")]
  #[strum(serialize = "developers.description")]
  DevelopersDescription,

  #[serde(rename = "developers.id")]
  #[strum(serialize = "developers.id")]
  DevelopersId,

  #[serde(rename = "developers.lang")]
  #[strum(serialize = "developers.lang")]
  DevelopersLang,

  #[serde(rename = "developers.name")]
  #[strum(serialize = "developers.name")]
  DevelopersName,

  #[serde(rename = "developers.original")]
  #[strum(serialize = "developers.original")]
  DevelopersOriginal,

  #[serde(rename = "developers.type")]
  #[strum(serialize = "developers.type")]
  DevelopersType,

  #[serde(rename = "devstatus")]
  #[strum(serialize = "devstatus")]
  DevStatus,

  #[serde(rename = "editions.eid")]
  #[strum(serialize = "editions.eid")]
  EditionsEid,

  #[serde(rename = "editions.lang")]
  #[strum(serialize = "editions.lang")]
  EditionsLang,

  #[serde(rename = "editions.name")]
  #[strum(serialize = "editions.name")]
  EditionsName,

  #[serde(rename = "editions.official")]
  #[strum(serialize = "editions.official")]
  EditionsOfficial,

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

  #[serde(rename = "image.dims")]
  #[strum(serialize = "image.dims")]
  ImageDims,

  #[serde(rename = "image.id")]
  #[strum(serialize = "image.id")]
  ImageId,

  #[serde(rename = "image.sexual")]
  #[strum(serialize = "image.sexual")]
  ImageSexual,

  #[serde(rename = "image.thumbnail")]
  #[strum(serialize = "image.thumbnail")]
  ImageThumbnail,

  #[serde(rename = "image.thumbnail_dims")]
  #[strum(serialize = "image.thumbnail_dims")]
  ImageThumbnailDims,

  #[serde(rename = "image.url")]
  #[strum(serialize = "image.url")]
  ImageUrl,

  #[serde(rename = "image.violence")]
  #[strum(serialize = "image.violence")]
  ImageViolence,

  #[serde(rename = "image.votecount")]
  #[strum(serialize = "image.votecount")]
  ImageVoteCount,

  #[serde(rename = "languages")]
  #[strum(serialize = "languages")]
  Languages,

  #[serde(rename = "length")]
  #[strum(serialize = "length")]
  Length,

  #[serde(rename = "length_minutes")]
  #[strum(serialize = "length_minutes")]
  LengthMinutes,

  #[serde(rename = "length_votes")]
  #[strum(serialize = "length_votes")]
  LengthVotes,

  #[serde(rename = "olang")]
  #[strum(serialize = "olang")]
  OLang,

  #[serde(rename = "platforms")]
  #[strum(serialize = "platforms")]
  Platforms,

  #[serde(rename = "rating")]
  #[strum(serialize = "rating")]
  Rating,

  #[serde(rename = "relations.id")]
  #[strum(serialize = "relations.id")]
  RelationsId,

  #[serde(rename = "relations.relation")]
  #[strum(serialize = "relations.relation")]
  RelationsRelation,

  #[serde(rename = "relations.relation_official")]
  #[strum(serialize = "relations.relation_official")]
  RelationsRelationOfficial,

  #[serde(rename = "released")]
  #[strum(serialize = "released")]
  Released,

  #[serde(rename = "screenshots.dims")]
  #[strum(serialize = "screenshots.dims")]
  ScreenshotsDims,

  #[serde(rename = "screenshots.id")]
  #[strum(serialize = "screenshots.id")]
  ScreenshotsId,

  #[serde(rename = "screenshots.sexual")]
  #[strum(serialize = "screenshots.sexual")]
  ScreenshotsSexual,

  #[serde(rename = "screenshots.thumbnail")]
  #[strum(serialize = "screenshots.thumbnail")]
  ScreenshotsThumbnail,

  #[serde(rename = "screenshots.thumbnail_dims")]
  #[strum(serialize = "screenshots.thumbnail_dims")]
  ScreenshotsThumbnailDims,

  #[serde(rename = "screenshots.url")]
  #[strum(serialize = "screenshots.url")]
  ScreenshotsUrl,

  #[serde(rename = "screenshots.violence")]
  #[strum(serialize = "screenshots.violence")]
  ScreenshotsViolence,

  #[serde(rename = "screenshots.votecount")]
  #[strum(serialize = "screenshots.votecount")]
  ScreenshotsVoteCount,

  #[serde(rename = "staff.eid")]
  #[strum(serialize = "staff.eid")]
  StaffEid,

  #[serde(rename = "staff.note")]
  #[strum(serialize = "staff.note")]
  StaffNote,

  #[serde(rename = "staff.role")]
  #[strum(serialize = "staff.role")]
  StaffRole,

  #[serde(rename = "tags.id")]
  #[strum(serialize = "tags.id")]
  TagsId,

  #[serde(rename = "tags.lie")]
  #[strum(serialize = "tags.lie")]
  TagsLie,

  #[serde(rename = "tags.rating")]
  #[strum(serialize = "tags.rating")]
  TagsRating,

  #[serde(rename = "tags.spoiler")]
  #[strum(serialize = "tags.spoiler")]
  TagsSpoiler,

  #[serde(rename = "title")]
  #[strum(serialize = "title")]
  Title,

  #[serde(rename = "titles.lang")]
  #[strum(serialize = "titles.lang")]
  TitlesLang,

  #[serde(rename = "titles.latin")]
  #[strum(serialize = "titles.latin")]
  TitlesLatin,

  #[serde(rename = "titles.main")]
  #[strum(serialize = "titles.main")]
  TitlesMain,

  #[serde(rename = "titles.official")]
  #[strum(serialize = "titles.official")]
  TitlesOfficial,

  #[serde(rename = "titles.title")]
  #[strum(serialize = "titles.title")]
  TitlesTitle,

  #[serde(rename = "va.note")]
  #[strum(serialize = "va.note")]
  VaNote,

  #[serde(rename = "votecount")]
  #[strum(serialize = "votecount")]
  VoteCount,
}

impl QueryField for VisualNovelField {}

impl_into_field_set!(VisualNovelField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SortVisualNovelBy {
  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "rating")]
  #[strum(serialize = "rating")]
  Rating,

  #[serde(rename = "released")]
  #[strum(serialize = "released")]
  Released,

  #[serde(rename = "searchrank")]
  #[strum(serialize = "searchrank")]
  SearchRank,

  #[serde(rename = "title")]
  #[strum(serialize = "title")]
  Title,

  #[serde(rename = "votecount")]
  #[strum(serialize = "votecount")]
  VoteCount,
}

impl SortQueryBy for SortVisualNovelBy {}
