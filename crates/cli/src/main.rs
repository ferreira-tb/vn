use anyhow::{Result, bail};
use clap::Parser;
use itertools::Itertools;
use serde::Serialize;
use serde_json::{Value as JsonValue, to_string_pretty};
use vn_core::http::FieldSet;
use vn_core::{
  CharacterField, ProducerField, ReleaseField, StaffField, TagField, TraitField, VisualNovelField,
  Vndb, VndbId,
};

#[derive(Parser)]
struct Cli {
  id: VndbId,

  #[arg(short = 'f', long)]
  field: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Cli::parse();
  let vndb = Vndb::new();

  macro_rules! fetch_and_print {
    ($id:expr, $find_fn:ident, $field_type:ident) => {
      let fields = if args.field.is_empty() {
        $field_type::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(value) = vndb
        .$find_fn(&$id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
        .map(|it| to_value(&it))
        .transpose()?
      {
        println!("{}", to_string_pretty(&value)?);
      } else {
        let id = $id;
        bail!("not found: {id}");
      }
    };
  }

  match args.id {
    VndbId::Character(id) => {
      fetch_and_print!(id, find_character, CharacterField);
    }
    VndbId::Producer(id) => {
      fetch_and_print!(id, find_producer, ProducerField);
    }
    VndbId::Release(id) => {
      fetch_and_print!(id, find_release, ReleaseField);
    }
    VndbId::Staff(id) => {
      fetch_and_print!(id, find_staff, StaffField);
    }
    VndbId::Tag(id) => {
      fetch_and_print!(id, find_tag, TagField);
    }
    VndbId::Trait(id) => {
      fetch_and_print!(id, find_trait, TraitField);
    }
    VndbId::User(id) => {
      if let Some(user) = vndb.find_user(&id).await? {
        println!("{}", to_string_pretty(&user)?);
      } else {
        bail!("not found: {id}");
      }
    }
    VndbId::VisualNovel(id) => {
      fetch_and_print!(id, find_visual_novel, VisualNovelField);
    }
  }

  Ok(())
}

fn to_value<T: Serialize>(value: &T) -> Result<JsonValue> {
  let mut value = serde_json::to_value(value)?;
  strip_null_values(&mut value);
  Ok(value)
}

fn strip_null_values(json: &mut JsonValue) {
  match json {
    JsonValue::Array(values) => {
      for value in values {
        strip_null_values(value);
      }
    }
    JsonValue::Object(map) => {
      let keys_to_remove = map
        .iter_mut()
        .filter_map(|(key, value)| {
          if value.is_null() {
            Some(key.clone())
          } else {
            strip_null_values(value);
            None
          }
        })
        .collect_vec();

      for key in keys_to_remove {
        map.remove(&key);
      }
    }
    _ => {}
  }
}
