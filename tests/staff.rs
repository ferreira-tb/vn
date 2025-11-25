use vn::StaffField::{Gender, IsMain, Lang, Name};
use vn::{Language, StaffGender, StaffId, Vndb};

const SUMIRE: &str = "Uesaka Sumire";
const SUMIRE_ID: u16 = 4466;

#[tokio::test]
async fn get_staff() {
  let vndb = Vndb::new();
  let filters = r#"["id", "=", "s4466"]"#;
  let results = vndb
    .post()
    .staff()
    .filters(filters.try_into().unwrap())
    .fields([Name, Lang, Gender, IsMain])
    .results(5)
    .send()
    .await
    .unwrap()
    .results;

  let staff = results
    .iter()
    .find(|staff| staff.ismain.unwrap_or(false))
    .unwrap();

  let name = staff.name.as_deref().unwrap();
  let lang = staff.lang.as_ref().unwrap();
  let gender = staff.gender.as_ref().unwrap();

  assert_eq!(staff.id, StaffId::from(SUMIRE_ID));
  assert!(name.eq_ignore_ascii_case(SUMIRE));
  assert_eq!(lang, &Language::Japanese);
  assert_eq!(gender, &StaffGender::Female);
}

#[tokio::test]
async fn find_staff() {
  let staff = Vndb::new()
    .find_staff(SUMIRE_ID)
    .fields([Name, Lang, Gender, IsMain])
    .send()
    .await
    .unwrap()
    .swap_remove(0);

  assert_eq!(staff.id, StaffId::from(SUMIRE_ID));
  assert_eq!(staff.name.as_deref(), Some(SUMIRE));
  assert_eq!(staff.lang, Some(Language::Japanese));
}

#[tokio::test]
async fn search_staff() {
  let staff = Vndb::new()
    .search_staff(SUMIRE.to_ascii_lowercase())
    .fields([Name, Lang, Gender, IsMain])
    .send()
    .await
    .unwrap()
    .swap_remove(0);

  assert_eq!(staff.id, StaffId::from(SUMIRE_ID));
  assert_eq!(staff.name.as_deref(), Some(SUMIRE));
  assert_eq!(staff.lang, Some(Language::Japanese));
}
