use vn::Vndb;

#[tokio::test]
async fn get_stats() {
  let stats = Vndb::new().get().stats().await;
  assert!(stats.is_ok());
}
