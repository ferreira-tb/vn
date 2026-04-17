use std::sync::LazyLock;
use vn::ProducerField::*;
use vn::{ProducerId, ProducerType, SortProducerBy, Vndb};

const YUZUSOFT: &str = "Yuzusoft";

static YUZUSOFT_ID: LazyLock<ProducerId> = LazyLock::new(|| {
  let id = format!("{}{}", ProducerId::PREFIX, 98);
  ProducerId::new(&id).unwrap()
});

#[tokio::test]
async fn get_producer() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "p98"]"#;
  let mut results = vndb
    .post()
    .producer()
    .filters(filters.try_into().unwrap())
    .fields([Name, Type])
    .sort(SortProducerBy::Name)
    .results(5)
    .reverse()
    .send()
    .await
    .unwrap()
    .results;

  assert_eq!(results.len(), 1);

  let producer = results.pop_front().unwrap();
  let name = producer.name.as_deref().unwrap();
  let producer_type = producer.r#type.as_ref().unwrap();

  assert_eq!(&producer.id, &*YUZUSOFT_ID);
  assert!(name.eq_ignore_ascii_case(YUZUSOFT));
  assert_eq!(producer_type, &ProducerType::Company);
}

#[tokio::test]
async fn find_producer() {
  let producer = Vndb::new()
    .find_producer(&*YUZUSOFT_ID)
    .fields([Name, Type])
    .send()
    .await
    .unwrap()
    .results
    .pop_front()
    .unwrap();

  assert_eq!(&producer.id, &*YUZUSOFT_ID);
  assert_eq!(producer.name.as_deref(), Some(YUZUSOFT));
  assert_eq!(producer.r#type, Some(ProducerType::Company));
}

#[tokio::test]
async fn search_producer() {
  let producer = Vndb::new()
    .search_producer(YUZUSOFT.to_ascii_uppercase())
    .fields([Name, Type])
    .send()
    .await
    .unwrap()
    .results
    .pop_front()
    .unwrap();

  assert_eq!(&producer.id, &*YUZUSOFT_ID);
  assert_eq!(producer.name.as_deref(), Some(YUZUSOFT));
  assert_eq!(producer.r#type, Some(ProducerType::Company));
}
