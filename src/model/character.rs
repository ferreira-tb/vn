use super::r#trait::Trait;
use super::release::Release;
use super::visual_novel::VisualNovel;
use super::{QueryField, SortQueryBy};
use crate::{impl_id_newtype, impl_id_newtype_from_numeric, impl_into_field_set};
use serde::{Deserialize, Deserializer, Serialize};
use std::result::Result as StdResult;
use strum::{Display, EnumIs, VariantArray};

#[cfg(feature = "regex")]
use {crate::impl_id_newtype_regex, regex::Regex, std::sync::LazyLock};

#[cfg(feature = "regex")]
static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^c\d+$").unwrap());

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Character {
  pub age: Option<u32>,
  pub aliases: Option<Vec<String>>,
  pub birthday: Option<CharacterBirthday>,
  pub blood_type: Option<String>,
  pub bust: Option<u32>,
  pub cup: Option<String>,
  pub description: Option<String>,
  pub height: Option<u32>,
  pub hips: Option<u32>,
  pub id: CharacterId,
  pub image: Option<CharacterImage>,
  pub name: Option<String>,
  pub original: Option<String>,
  pub sex: Option<CharacterSex>,
  pub traits: Option<Vec<CharacterTrait>>,
  pub vns: Option<Vec<CharacterVisualNovel>>,
  pub waist: Option<u32>,
  pub weight: Option<u32>,
}

impl From<Character> for CharacterId {
  fn from(c: Character) -> Self {
    c.id
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterId(String);

impl CharacterId {
  pub const PREFIX: &'static str = "c";
}

impl_id_newtype!(CharacterId);
impl_id_newtype_from_numeric!(CharacterId::PREFIX, CharacterId);

#[cfg(feature = "regex")]
impl_id_newtype_regex!(CharacterId, ID_REGEX);

#[remain::sorted]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterBirthday {
  pub day: u32,
  pub month: u32,
}

impl<'de> Deserialize<'de> for CharacterBirthday {
  fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let date: [u32; 2] = Deserialize::deserialize(deserializer)?;
    Ok(Self { day: date[1], month: date[0] })
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterImage {
  pub dims: Option<[u32; 2]>,
  pub id: Option<String>,
  pub sexual: Option<f32>,
  pub url: Option<String>,
  pub violence: Option<f32>,
  pub votecount: Option<u32>,
}

#[remain::sorted]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterSex {
  pub apparent: Option<CharacterSexValue>,
  pub real: Option<CharacterSexValue>,
}

impl<'de> Deserialize<'de> for CharacterSex {
  fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    type Value = Option<CharacterSexValue>;
    let array: [Value; 2] = Deserialize::deserialize(deserializer)?;
    Ok(Self { apparent: array[0], real: array[1] })
  }
}

#[remain::sorted]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Display, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum CharacterSexValue {
  #[serde(rename = "b")]
  #[strum(serialize = "b")]
  Both,

  #[serde(rename = "f")]
  #[strum(serialize = "f")]
  Female,

  #[serde(rename = "m")]
  #[strum(serialize = "m")]
  Male,

  #[serde(rename = "n")]
  #[strum(serialize = "n")]
  None,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterTrait {
  pub lie: Option<bool>,
  #[serde(flatten)]
  pub r#trait: Trait,
  pub spoiler: Option<u8>,
}

impl From<CharacterTrait> for Trait {
  fn from(ct: CharacterTrait) -> Self {
    ct.r#trait
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterVisualNovel {
  pub release: Option<Release>,
  pub role: Option<String>,
  pub spoiler: Option<u8>,
  #[serde(flatten)]
  pub visual_novel: VisualNovel,
}

impl From<CharacterVisualNovel> for VisualNovel {
  fn from(cvn: CharacterVisualNovel) -> Self {
    cvn.visual_novel
  }
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum CharacterField {
  #[serde(rename = "age")]
  #[strum(serialize = "age")]
  Age,

  #[serde(rename = "aliases")]
  #[strum(serialize = "aliases")]
  Aliases,

  #[serde(rename = "birthday")]
  #[strum(serialize = "birthday")]
  Birthday,

  #[serde(rename = "blood_type")]
  #[strum(serialize = "blood_type")]
  BloodType,

  #[serde(rename = "bust")]
  #[strum(serialize = "bust")]
  Bust,

  #[serde(rename = "cup")]
  #[strum(serialize = "cup")]
  Cup,

  #[serde(rename = "description")]
  #[strum(serialize = "description")]
  Description,

  #[serde(rename = "height")]
  #[strum(serialize = "height")]
  Height,

  #[serde(rename = "hips")]
  #[strum(serialize = "hips")]
  Hips,

  #[serde(rename = "id")]
  #[strum(serialize = "id")]
  Id,

  #[serde(rename = "image.id")]
  #[strum(serialize = "image.id")]
  ImageId,

  #[serde(rename = "image.url")]
  #[strum(serialize = "image.url")]
  ImageUrl,

  #[serde(rename = "name")]
  #[strum(serialize = "name")]
  Name,

  #[serde(rename = "original")]
  #[strum(serialize = "original")]
  Original,

  #[serde(rename = "sex")]
  #[strum(serialize = "sex")]
  Sex,

  #[serde(rename = "traits.id")]
  #[strum(serialize = "traits.id")]
  TraitId,

  #[serde(rename = "traits.lie")]
  #[strum(serialize = "traits.lie")]
  TraitLie,

  #[serde(rename = "traits.spoiler")]
  #[strum(serialize = "traits.spoiler")]
  TraitSpoiler,

  #[serde(rename = "vns.aliases")]
  #[strum(serialize = "vns.aliases")]
  VisualNovelAliases,

  #[serde(rename = "vns.alttitle")]
  #[strum(serialize = "vns.alttitle")]
  VisualNovelAltTitle,

  #[serde(rename = "vns.id")]
  #[strum(serialize = "vns.id")]
  VisualNovelId,

  #[serde(rename = "vns.release.id")]
  #[strum(serialize = "vns.release.id")]
  VisualNovelReleaseId,

  #[serde(rename = "vns.role")]
  #[strum(serialize = "vns.role")]
  VisualNovelRole,

  #[serde(rename = "vns.spoiler")]
  #[strum(serialize = "vns.spoiler")]
  VisualNovelSpoiler,

  #[serde(rename = "vns.title")]
  #[strum(serialize = "vns.title")]
  VisualNovelTitle,

  #[serde(rename = "waist")]
  #[strum(serialize = "waist")]
  Waist,

  #[serde(rename = "weight")]
  #[strum(serialize = "weight")]
  Weight,
}

impl QueryField for CharacterField {}

impl_into_field_set!(CharacterField);

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum SortCharacterBy {
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

impl SortQueryBy for SortCharacterBy {}
