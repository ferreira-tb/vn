use vn::ProducerField::{Name, Type};
use vn::{ProducerId, ProducerType, SortProducerBy, Vndb};

const TRIANGLE: &str = "Triangle";
const TRIANGLE_ID: u16 = 332;

#[tokio::test]
async fn get_producer() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "p332"]"#;
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

  let producer = results.swap_remove(0);
  let name = producer.name.as_deref().unwrap();
  let producer_type = producer.r#type.as_ref().unwrap();

  assert_eq!(producer.id, ProducerId::from(TRIANGLE_ID));
  assert!(name.eq_ignore_ascii_case(TRIANGLE));
  assert_eq!(producer_type, &ProducerType::Company);
}

#[tokio::test]
async fn find_producer() {
  let producer = Vndb::new()
    .find_producer(TRIANGLE_ID)
    .fields([Name, Type])
    .send()
    .await
    .unwrap()
    .results
    .swap_remove(0);

  assert_eq!(producer.id, ProducerId::from(TRIANGLE_ID));
  assert_eq!(producer.name.as_deref(), Some(TRIANGLE));
  assert_eq!(producer.r#type, Some(ProducerType::Company));
}

#[tokio::test]
async fn search_producer() {
  let producer = Vndb::new()
    .search_producer(TRIANGLE.to_ascii_uppercase())
    .fields([Name, Type])
    .send()
    .await
    .unwrap()
    .results
    .swap_remove(0);

  assert_eq!(producer.id, ProducerId::from(TRIANGLE_ID));
  assert_eq!(producer.name.as_deref(), Some(TRIANGLE));
  assert_eq!(producer.r#type, Some(ProducerType::Company));
}
