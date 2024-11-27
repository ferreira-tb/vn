use serde::{Deserialize, Serialize};
use strum::Display;

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AuthInfo {
  pub id: String,
  pub permissions: Vec<TokenPermission>,
  pub username: String,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TokenPermission {
  #[serde(rename = "listread")]
  #[strum(serialize = "listread")]
  ListRead,

  #[serde(rename = "listwrite")]
  #[strum(serialize = "listwrite")]
  ListWrite,
}
