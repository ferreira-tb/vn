use vn::VisualNovelField::{Title, TitlesMain, TitlesTitle};
use vn::{VisualNovelId, Vndb};

const NOVEL: &str = "Yosuga no Sora";
const NOVEL_JP: &str = "ヨスガノソラ";
const NOVEL_ID: u16 = 1194;

#[tokio::test]
async fn get_visual_novel() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "v1194"]"#;
  let visual_novel = vndb
    .post()
    .visual_novel()
    .filters(filters.try_into().unwrap())
    .fields([Title, TitlesMain, TitlesTitle])
    .results(5)
    .send()
    .await
    .unwrap();

  let visual_novel = visual_novel
    .results
    .iter()
    .find(|it| {
      it.title
        .as_deref()
        .unwrap()
        .eq_ignore_ascii_case(NOVEL)
    })
    .unwrap();

  let title = visual_novel.title.as_deref().unwrap();
  assert_eq!(visual_novel.id, VisualNovelId::from(NOVEL_ID));
  assert!(title.eq_ignore_ascii_case(NOVEL));

  let main = visual_novel
    .titles
    .as_ref()
    .unwrap()
    .iter()
    .find(|it| it.main.unwrap_or(false))
    .unwrap();

  let title = main.title.as_deref().unwrap();
  assert!(title == NOVEL || title == NOVEL_JP);
}

#[tokio::test]
async fn find_visual_novel() {
  let visual_novel = Vndb::new()
    .find_visual_novel(NOVEL_ID)
    .fields(Title)
    .send()
    .await
    .unwrap()
    .results
    .swap_remove(0);

  assert_eq!(visual_novel.id, VisualNovelId::from(NOVEL_ID));
  assert_eq!(visual_novel.title.as_deref(), Some(NOVEL));
}

#[tokio::test]
async fn search_visual_novel() {
  let visual_novel = Vndb::new()
    .search_visual_novel(NOVEL.to_ascii_lowercase())
    .fields(Title)
    .send()
    .await
    .unwrap()
    .results
    .swap_remove(0);

  assert_eq!(visual_novel.id, VisualNovelId::from(NOVEL_ID));
  assert_eq!(visual_novel.title.as_deref(), Some(NOVEL));
}
