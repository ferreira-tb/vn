use vn::Vndb;

#[tokio::test]
async fn get_schema() {
  let schema = Vndb::new().get().schema().await;
  assert!(schema.is_ok());
}
