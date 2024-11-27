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
  pub use super::r#trait::{SortTraitBy, Trait, TraitField, TraitId};
  pub use super::release::{
    ExternalLink, Release, ReleaseField, ReleaseId, ReleaseImage, ReleaseImageType,
    ReleaseLanguage, ReleaseMedia, ReleaseProducer, ReleaseResolution, ReleaseType,
    ReleaseVisualNovel, ReleaseVoiced, SortReleaseBy,
  };
  pub use super::schema::{Language, Schema};
  pub use super::staff::{SortStaffBy, Staff, StaffAlias, StaffField, StaffGender, StaffId};
  pub use super::stats::Stats;
  pub use super::tag::{SortTagBy, Tag, TagCategory, TagField, TagId};
  pub use super::user::{User, UserField, UserId, UserUrlQuery, Users};
  pub use super::visual_novel::{
    SortVisualNovelBy, VisualNovel, VisualNovelDevStatus, VisualNovelDeveloper, VisualNovelEdition,
    VisualNovelField, VisualNovelId, VisualNovelImage, VisualNovelLength, VisualNovelRelation,
    VisualNovelScreenShot, VisualNovelStaff, VisualNovelTag, VisualNovelTitle,
    VisualNovelVoiceActor,
  };
  pub use super::Response;
}

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Response<T> {
  pub compact_filters: Option<String>,
  pub count: Option<u32>,
  pub more: bool,
  pub normalized_filters: Option<JsonValue>,
  pub results: Vec<T>,
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
