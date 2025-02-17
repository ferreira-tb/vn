use vn::TraitField::{GroupName, Name};
use vn::{TraitId, Vndb};

const AIRHEAD: &str = "Airhead";
const AIRHEAD_ID: u16 = 229;
const AIRHEAD_GROUP: &str = "Personality";

#[tokio::test]
async fn get_trait() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "i229"]"#;
  let mut results = vndb
    .post()
    .r#trait()
    .filters(filters.try_into().unwrap())
    .fields([Name, GroupName])
    .results(5)
    .send()
    .await
    .unwrap()
    .results;

  assert_eq!(results.len(), 1);

  let r#trait = results.swap_remove(0);
  let name = r#trait.name.as_deref().unwrap();
  let group_name = r#trait.group_name.as_deref().unwrap();

  assert_eq!(r#trait.id, TraitId::from(AIRHEAD_ID));
  assert!(name.eq_ignore_ascii_case(AIRHEAD));
  assert!(group_name.eq_ignore_ascii_case(AIRHEAD_GROUP));
}

#[tokio::test]
async fn find_trait() {
  let r#trait = Vndb::new()
    .find_trait(AIRHEAD_ID)
    .fields([Name, GroupName])
    .send()
    .await
    .unwrap()
    .results
    .swap_remove(0);

  assert_eq!(r#trait.id, TraitId::from(AIRHEAD_ID));
  assert_eq!(r#trait.name.as_deref(), Some(AIRHEAD));
  assert_eq!(r#trait.group_name.as_deref(), Some(AIRHEAD_GROUP));
}

#[tokio::test]
async fn search_trait() {
  let r#trait = Vndb::new()
    .search_trait(AIRHEAD.to_ascii_lowercase())
    .fields([Name, GroupName])
    .send()
    .await
    .unwrap()
    .results
    .swap_remove(0);

  assert_eq!(r#trait.id, TraitId::from(AIRHEAD_ID));
  assert_eq!(r#trait.name.as_deref(), Some(AIRHEAD));
  assert_eq!(r#trait.group_name.as_deref(), Some(AIRHEAD_GROUP));
}
