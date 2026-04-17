use std::sync::LazyLock;
use vn::ReleaseField::*;
use vn::{ReleaseId, Vndb};

const KUSARIHIME: &str = "Kusarihime ~Euthanasia~ Download Edition";

static KUSARIHIME_ID: LazyLock<ReleaseId> = LazyLock::new(|| {
  let id = format!("{}{}", ReleaseId::PREFIX, 80);
  ReleaseId::new(&id).unwrap()
});

#[tokio::test]
async fn get_release() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "r80"]"#;
  let results = vndb
    .post()
    .release()
    .filters(filters.try_into().unwrap())
    .fields([Title, AltTitle, ImagesUrl])
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
  assert_eq!(&release.id, &*KUSARIHIME_ID);
  assert!(title.eq_ignore_ascii_case(KUSARIHIME));

  assert!(
    release
      .images
      .as_ref()
      .is_some_and(|it| !it.is_empty())
  );
}

#[tokio::test]
async fn find_release() {
  let release = Vndb::new()
    .find_release(&*KUSARIHIME_ID)
    .fields([Title, AltTitle])
    .send()
    .await
    .unwrap()
    .results
    .pop_front()
    .unwrap();

  assert_eq!(&release.id, &*KUSARIHIME_ID);
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

  assert_eq!(&release.id, &*KUSARIHIME_ID);
  assert_eq!(release.title.as_deref(), Some(KUSARIHIME));
}
