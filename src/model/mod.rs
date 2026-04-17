pub mod auth_info;
pub mod character;
pub mod producer;
pub mod release;
pub mod schema;
pub mod staff;
pub mod stats;
pub mod tag;
pub mod r#trait;
pub mod user;
pub mod visual_novel;

pub mod prelude {
  pub use super::auth_info::{AuthInfo, TokenPermission};
  pub use super::character::{
    Character, CharacterBirthday, CharacterField, CharacterId, CharacterImage, CharacterSex,
    CharacterSexValue, CharacterTrait, CharacterVisualNovel, SortCharacterBy,
  };
  pub use super::producer::{Producer, ProducerField, ProducerId, ProducerType, SortProducerBy};
  pub use super::release::{
    ExternalLink, Release, ReleaseField, ReleaseId, ReleaseImage, ReleaseImageType,
    ReleaseLanguage, ReleaseMedia, ReleaseProducer, ReleaseResolution, ReleaseType,
    ReleaseVisualNovel, ReleaseVoiced, SortReleaseBy,
  };
  pub use super::schema::{Language, Schema};
  pub use super::staff::{SortStaffBy, Staff, StaffAlias, StaffField, StaffGender, StaffId};
  pub use super::stats::Stats;
  pub use super::tag::{SortTagBy, Tag, TagCategory, TagField, TagId};
  pub use super::r#trait::{SortTraitBy, Trait, TraitField, TraitId};
  pub use super::user::{User, UserField, UserId, UserUrlQuery, Users};
  pub use super::visual_novel::{
    SortVisualNovelBy, VisualNovel, VisualNovelDevStatus, VisualNovelDeveloper, VisualNovelEdition,
    VisualNovelField, VisualNovelId, VisualNovelImage, VisualNovelLength, VisualNovelRelation,
    VisualNovelScreenShot, VisualNovelStaff, VisualNovelTag, VisualNovelTitle,
    VisualNovelVoiceActor,
  };
  pub use super::{Response, VndbId};
}

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;
use strum::EnumIs;

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Response<T> {
  pub compact_filters: Option<String>,
  pub count: Option<u32>,
  pub more: bool,
  pub normalized_filters: Option<JsonValue>,
  pub results: VecDeque<T>,
}

impl<T> IntoIterator for Response<T> {
  type Item = T;
  type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.results.into_iter()
  }
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize, EnumIs)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum VndbId {
  Character(character::CharacterId),
  Producer(producer::ProducerId),
  Release(release::ReleaseId),
  Staff(staff::StaffId),
  Tag(tag::TagId),
  Trait(r#trait::TraitId),
  User(user::UserId),
  VisualNovel(visual_novel::VisualNovelId),
}

impl VndbId {
  pub fn new(id: impl AsRef<str>) -> Option<Self> {
    Self::try_from(id.as_ref()).ok()
  }
}

impl FromStr for VndbId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let prefix = s
      .chars()
      .next()
      .ok_or_else(|| Error::InvalidId(s.to_owned()))?;

    match prefix {
      character::CharacterId::PREFIX => Ok(Self::Character(s.parse()?)),
      producer::ProducerId::PREFIX => Ok(Self::Producer(s.parse()?)),
      release::ReleaseId::PREFIX => Ok(Self::Release(s.parse()?)),
      staff::StaffId::PREFIX => Ok(Self::Staff(s.parse()?)),
      tag::TagId::PREFIX => Ok(Self::Tag(s.parse()?)),
      r#trait::TraitId::PREFIX => Ok(Self::Trait(s.parse()?)),
      user::UserId::PREFIX => Ok(Self::User(s.parse()?)),
      visual_novel::VisualNovelId::PREFIX => Ok(Self::VisualNovel(s.parse()?)),
      _ => Err(Error::InvalidId(s.to_owned())),
    }
  }
}

impl TryFrom<&str> for VndbId {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self> {
    value.parse()
  }
}

impl fmt::Display for VndbId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Character(id) => id.fmt(f),
      Self::Producer(id) => id.fmt(f),
      Self::Release(id) => id.fmt(f),
      Self::Staff(id) => id.fmt(f),
      Self::Tag(id) => id.fmt(f),
      Self::Trait(id) => id.fmt(f),
      Self::User(id) => id.fmt(f),
      Self::VisualNovel(id) => id.fmt(f),
    }
  }
}

pub trait QueryField: fmt::Display + sealed::Sealed {}

pub trait SortQueryBy: fmt::Display + sealed::Sealed {}

mod sealed {
  pub trait Sealed {}

  // Field
  impl Sealed for super::character::CharacterField {}
  impl Sealed for super::producer::ProducerField {}
  impl Sealed for super::release::ReleaseField {}
  impl Sealed for super::staff::StaffField {}
  impl Sealed for super::tag::TagField {}
  impl Sealed for super::r#trait::TraitField {}
  impl Sealed for super::user::UserField {}
  impl Sealed for super::visual_novel::VisualNovelField {}

  // Sort
  impl Sealed for super::character::SortCharacterBy {}
  impl Sealed for super::producer::SortProducerBy {}
  impl Sealed for super::release::SortReleaseBy {}
  impl Sealed for super::staff::SortStaffBy {}
  impl Sealed for super::tag::SortTagBy {}
  impl Sealed for super::r#trait::SortTraitBy {}
  impl Sealed for super::visual_novel::SortVisualNovelBy {}
}
