use vn::UserField::{self, LengthVotes};
use vn::{UserId, Vndb};

#[tokio::test]
async fn get_user() {
  let vndb = Vndb::new();
  let users = vndb
    .get()
    .user(500, UserField::all())
    .await
    .unwrap();

  let user = users.get("u500").unwrap();
  assert_eq!(&*user.id, "u500");
  assert!(user.lengthvotes.is_some());
  assert!(user.lengthvotes_sum.is_some());

  let users = vndb
    .get()
    .user("u1000", LengthVotes)
    .await
    .unwrap();

  let user = users.get("u1000").unwrap();
  assert_eq!(&*user.id, "u1000");
  assert!(user.lengthvotes.is_some());
  assert!(user.lengthvotes_sum.is_none());
}

#[tokio::test]
async fn get_user_range() {
  let vndb = Vndb::new();
  let users = vndb
    .get()
    .user(500..510, UserField::none())
    .await
    .unwrap();

  assert_eq!(users.len(), 10);
  assert!(users.get("u500").is_some());
  assert!(users.get("u509").is_some());

  let users = vndb
    .get()
    .user(700..=710, LengthVotes)
    .await
    .unwrap();

  assert_eq!(users.len(), 11);
  assert!(
    users
      .get("u700")
      .unwrap()
      .lengthvotes
      .is_some()
  );
  assert!(
    users
      .get("u710")
      .unwrap()
      .lengthvotes
      .is_some()
  );
}

#[tokio::test]
async fn find_user() {
  let user = Vndb::new()
    .find_user(500)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(user.id, UserId::from(500));
  assert!(user.lengthvotes.is_some());
  assert!(user.lengthvotes_sum.is_some());
}
