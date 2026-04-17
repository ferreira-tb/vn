use anyhow::Result;
use clap::Parser;
use serde_json::to_string_pretty;
use vn::http::FieldSet;
use vn::{
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
#[expect(clippy::too_many_lines)]
async fn main() -> Result<()> {
  let args = Cli::parse();
  let vndb = Vndb::new();

  match args.id {
    VndbId::Character(id) => {
      let fields = if args.field.is_empty() {
        CharacterField::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(character) = vndb
        .find_character(&id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
      {
        println!("{}", to_string_pretty(&character)?);
      } else {
        println!("character not found: {id}");
      }
    }
    VndbId::Producer(id) => {
      let fields = if args.field.is_empty() {
        ProducerField::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(producer) = vndb
        .find_producer(&id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
      {
        println!("{}", to_string_pretty(&producer)?);
      } else {
        println!("producer not found: {id}");
      }
    }
    VndbId::Release(id) => {
      let fields = if args.field.is_empty() {
        ReleaseField::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(release) = vndb
        .find_release(&id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
      {
        println!("{}", to_string_pretty(&release)?);
      } else {
        println!("release not found: {id}");
      }
    }
    VndbId::Staff(id) => {
      let fields = if args.field.is_empty() {
        StaffField::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(staff) = vndb
        .find_staff(&id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
      {
        println!("{}", to_string_pretty(&staff)?);
      } else {
        println!("staff not found: {id}");
      }
    }
    VndbId::Tag(id) => {
      let fields = if args.field.is_empty() {
        TagField::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(tag) = vndb
        .find_tag(&id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
      {
        println!("{}", to_string_pretty(&tag)?);
      } else {
        println!("tag not found: {id}");
      }
    }
    VndbId::Trait(id) => {
      let fields = if args.field.is_empty() {
        TraitField::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(r#trait) = vndb
        .find_trait(&id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
      {
        println!("{}", to_string_pretty(&r#trait)?);
      } else {
        println!("trait not found: {id}");
      }
    }
    VndbId::User(id) => {
      if let Some(user) = vndb.find_user(&id).await? {
        println!("{}", to_string_pretty(&user)?);
      } else {
        println!("user not found: {id}");
      }
    }
    VndbId::VisualNovel(id) => {
      let fields = if args.field.is_empty() {
        VisualNovelField::all()
      } else {
        FieldSet::from_raw(args.field)
      };

      if let Some(visual_novel) = vndb
        .find_visual_novel(&id)
        .fields(fields)
        .send()
        .await?
        .results
        .pop_front()
      {
        println!("{}", to_string_pretty(&visual_novel)?);
      } else {
        println!("visual novel not found: {id}");
      }
    }
  }

  Ok(())
}
