use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value as JsonValue;
use std::result::Result as StdResult;
use strum::Display;

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Schema {
  pub api_fields: JsonValue,
  pub enums: SchemaEnum,
  pub extlinks: JsonValue,
}

#[remain::sorted]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SchemaEnum {
  pub language: JsonValue,
  pub medium: JsonValue,
  pub platform: JsonValue,
  pub staff_role: JsonValue,
}

#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash, Display)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Language {
  #[serde(rename = "zh")]
  #[strum(serialize = "zh")]
  Chinese,

  #[serde(rename = "en")]
  #[strum(serialize = "en")]
  English,

  #[serde(rename = "ja")]
  #[strum(serialize = "ja")]
  Japanese,

  #[serde(rename = "ko")]
  #[strum(serialize = "ko")]
  Korean,

  #[serde(rename = "pt")]
  #[strum(serialize = "pt")]
  Portuguese,

  #[serde(rename = "ru")]
  #[strum(serialize = "ru")]
  Russian,

  #[serde(rename = "es")]
  #[strum(serialize = "es")]
  Spanish,

  Unknown(String),
}

impl<'de> Deserialize<'de> for Language {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
    let s = String::deserialize(deserializer)?;
    Ok(match s.as_str() {
      "zh" | "zh-Hans" | "zh-Hant" => Language::Chinese,
      "en" => Language::English,
      "ja" => Language::Japanese,
      "ko" => Language::Korean,
      "pt-br" | "pt-pt" => Language::Portuguese,
      "ru" => Language::Russian,
      "es" => Language::Spanish,
      _ => Language::Unknown(s),
    })
  }
}
