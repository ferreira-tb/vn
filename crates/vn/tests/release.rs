use vn::ReleaseField::{AltTitle, Title};
use vn::{ReleaseId, Vndb};

const KUSARIHIME: &str = "Kusarihime ~Euthanasia~ Download Edition";
const KUSARIHIME_ID: u16 = 80;

#[tokio::test]
async fn get_release() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "r80"]"#;
  let results = vndb
    .post()
    .release()
    .filters(filters.try_into().unwrap())
    .fields([Title, AltTitle])
    .results(5)
    .send()
    .await
    .unwrap()
    .results;

  let release = results
    .iter()
    .find(|it| {
      it.title
        .as_deref()
        .unwrap()
        .eq_ignore_ascii_case(KUSARIHIME)
    })
    .unwrap();

  let title = release.title.as_deref().unwrap();
  assert_eq!(release.id, ReleaseId::from(KUSARIHIME_ID));
  assert!(title.eq_ignore_ascii_case(KUSARIHIME));
}

#[tokio::test]
async fn find_release() {
  let release = Vndb::new()
    .find_release(KUSARIHIME_ID)
    .fields([Title, AltTitle])
    .send()
    .await
    .unwrap()
    .results
    .swap_remove(0);

  assert_eq!(release.id, ReleaseId::from(KUSARIHIME_ID));
  assert_eq!(release.title.as_deref(), Some(KUSARIHIME));
}

#[tokio::test]
async fn search_release() {
  let results = Vndb::new()
    .search_release("Kusarihime")
    .fields([Title, AltTitle])
    .send()
    .await
    .unwrap()
    .results;

  let release = results
    .iter()
    .find(|it| {
      it.title
        .as_deref()
        .unwrap()
        .eq_ignore_ascii_case(KUSARIHIME)
    })
    .unwrap();

  assert_eq!(release.id, ReleaseId::from(KUSARIHIME_ID));
  assert_eq!(release.title.as_deref(), Some(KUSARIHIME));
}
