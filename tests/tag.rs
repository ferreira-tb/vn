use std::sync::LazyLock;
use vn::TagField::*;
use vn::{TagCategory, TagId, Vndb};

const FANTASY: &str = "Medieval Fantasy";

static FANTASY_ID: LazyLock<TagId> = LazyLock::new(|| {
  let id = format!("{}{}", TagId::PREFIX, 994);
  TagId::new(&id).unwrap()
});

#[tokio::test]
async fn get_tag() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "g994"]"#;
  let mut results = vndb
    .post()
    .tag()
    .filters(filters.try_into().unwrap())
    .fields([Name, Category])
    .results(5)
    .send()
    .await
    .unwrap()
    .results;

  assert_eq!(results.len(), 1);

  let tag = results.pop_front().unwrap();
  let name = tag.name.as_deref().unwrap();
  let category = tag.category.as_ref().unwrap();

  assert_eq!(&tag.id, &*FANTASY_ID);
  assert!(name.eq_ignore_ascii_case(FANTASY));
  assert_eq!(category, &TagCategory::Content);
}

#[tokio::test]
async fn find_tag() {
  let tag = Vndb::new()
    .find_tag(&*FANTASY_ID)
    .fields([Name, Category])
    .send()
    .await
    .unwrap()
    .pop_front()
    .unwrap();

  assert_eq!(&tag.id, &*FANTASY_ID);
  assert_eq!(tag.name.as_deref(), Some(FANTASY));
  assert_eq!(tag.category, Some(TagCategory::Content));
}

#[tokio::test]
async fn search_tag() {
  let tag = Vndb::new()
    .search_tag(FANTASY.to_ascii_lowercase())
    .fields([Name, Category])
    .send()
    .await
    .unwrap()
    .pop_front()
    .unwrap();

  assert_eq!(&tag.id, &*FANTASY_ID);
  assert_eq!(tag.name.as_deref(), Some(FANTASY));
  assert_eq!(tag.category, Some(TagCategory::Content));
}
