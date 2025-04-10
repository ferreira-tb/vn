# vn

```toml
[dependencies]
vn = 0.3
```

## Usage

Create a new VNDB client:

```rust
use vn::Vndb;
use std::time::Duration;

let vndb = Vndb::builder()
  .max_concurrent_requests(5)
  .timeout(Duration::from_secs(10))
  .token("abcd-efghi-jklmn-opqrs-tuvx-z1234-5678")
  .user_agent("some-user-agent/v5.0")
  .build();
```

Find a visual novel by its id:

```rust
use vn::{Vndb, VisualNovelField};

let vndb = Vndb::new();
let results = vndb
  .find_visual_novel(1194)
  .fields(VisualNovelField::Title)
  .send()
  .await
  .unwrap()
  .results;

let visual_novel = results.first().unwrap();
let title = visual_novel.title.as_deref().unwrap();
assert!(title.eq_ignore_ascii_case("Yosuga no Sora"));
```

Search characters by their name:

```rust
use vn::{Vndb, CharacterField, SortCharacterBy};

let vndb = Vndb::new();
let results = vndb
  .search_character("Minazuki Shigure")
  .fields([CharacterField::Name, CharacterField::Original])
  .results(10)
  .sort(SortCharacterBy::Name)
  .send()
  .await
  .unwrap()
  .results;

let character = results.first().unwrap();
let name = character.name.as_deref().unwrap();
let original = character.original.as_deref().unwrap();

assert!(name.eq_ignore_ascii_case("Minazuki Shigure"));
assert_eq!(original, "水無月 時雨");
```
