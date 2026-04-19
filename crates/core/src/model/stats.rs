use serde::{Deserialize, Serialize};

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Stats {
  pub chars: u32,
  pub producers: u32,
  pub releases: u32,
  pub staff: u32,
  pub tags: u32,
  pub traits: u32,
  pub vn: u32,
}
