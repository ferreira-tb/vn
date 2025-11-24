mod query;
pub mod request;

pub use query::{FieldSet, JsonQuery, JsonQueryBuilder, JsonQueryFilter, UrlQueryParams};
pub use request::BASE_URL;
pub use request::get::Get;
pub use request::post::Post;
use serde::{Deserialize, Serialize};
use strum::{Display, VariantArray};
use url::Url;

#[non_exhaustive]
#[remain::sorted]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Display, VariantArray)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Endpoint {
  #[serde(rename = "authinfo")]
  #[strum(serialize = "authinfo")]
  AuthInfo,

  #[serde(rename = "character")]
  #[strum(serialize = "character")]
  Character,

  #[serde(rename = "producer")]
  #[strum(serialize = "producer")]
  Producer,

  #[serde(rename = "release")]
  #[strum(serialize = "release")]
  Release,

  #[serde(rename = "rlist")]
  #[strum(serialize = "rlist")]
  Rlist,

  #[serde(rename = "schema")]
  #[strum(serialize = "schema")]
  Schema,

  #[serde(rename = "staff")]
  #[strum(serialize = "staff")]
  Staff,

  #[serde(rename = "stats")]
  #[strum(serialize = "stats")]
  Stats,

  #[serde(rename = "tag")]
  #[strum(serialize = "tag")]
  Tag,

  #[serde(rename = "trait")]
  #[strum(serialize = "trait")]
  Trait,

  #[serde(rename = "ulist")]
  #[strum(serialize = "ulist")]
  Ulist,

  #[serde(rename = "ulist_labels")]
  #[strum(serialize = "ulist_labels")]
  UlistLabels,

  #[serde(rename = "user")]
  #[strum(serialize = "user")]
  User,

  #[serde(rename = "vn")]
  #[strum(serialize = "vn")]
  VisualNovel,
}

impl Endpoint {
  pub fn url(self) -> Url {
    Url::parse(&format!("{BASE_URL}/{self}")).unwrap()
  }
}
