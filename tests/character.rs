use std::sync::LazyLock;
use vn::CharacterField::*;
use vn::{CharacterId, Vndb};

const YUKARI: &str = "Kaburagi Yukari";
const YUKARI_JP: &str = "鏑木 紫";

static YUKARI_ID: LazyLock<CharacterId> = LazyLock::new(|| {
  let id = format!("{}{}", CharacterId::PREFIX, 81501);
  CharacterId::new(&id).unwrap()
});

#[tokio::test]
async fn get_character() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "c81501"]"#;
  let character = vndb
    .post()
    .character()
    .filters(filters.try_into().unwrap())
    .fields([
      Name,
      Original,
      ImageUrl,
      Description,
      VisualNovelAliases,
      VisualNovelAltTitle,
      VisualNovelTitle,
    ])
    .results(5)
    .send()
    .await
    .unwrap();

  let character = character
    .results
    .iter()
    .find(|it| {
      it.name
        .as_deref()
        .unwrap()
        .eq_ignore_ascii_case(YUKARI)
    })
    .unwrap();

  let name = character.name.as_deref().unwrap();
  assert_eq!(&character.id, &*YUKARI_ID);
  assert!(name.eq_ignore_ascii_case(YUKARI));
  assert_eq!(character.original.as_deref(), Some(YUKARI_JP));
}

#[tokio::test]
async fn find_character() {
  let character = Vndb::new()
    .find_character(&*YUKARI_ID)
    .fields([Name, Original])
    .send()
    .await
    .unwrap()
    .results
    .pop_front()
    .unwrap();

  assert_eq!(&character.id, &*YUKARI_ID);
  assert_eq!(character.name.as_deref(), Some(YUKARI));
  assert_eq!(character.original.as_deref(), Some(YUKARI_JP));
}

#[tokio::test]
async fn search_character() {
  let character = Vndb::new()
    .search_character(YUKARI.to_ascii_uppercase())
    .fields([Name, Original])
    .send()
    .await
    .unwrap()
    .results
    .into_iter()
    .find(|it| &it.id == &*YUKARI_ID)
    .unwrap();

  assert_eq!(character.name.as_deref(), Some(YUKARI));
  assert_eq!(character.original.as_deref(), Some(YUKARI_JP));
}
