#[doc(hidden)]
#[macro_export]
macro_rules! impl_string_set {
  ($target:ident) => {
    impl $target {
      pub fn new() -> Self {
        use ahash::{HashSet, HashSetExt};
        Self(HashSet::new())
      }

      pub fn with_capacity(capacity: usize) -> Self {
        use ahash::{HashSet, HashSetExt};
        Self(HashSet::with_capacity(capacity))
      }

      pub fn capacity(&self) -> usize {
        self.0.capacity()
      }

      pub fn clear(&mut self) {
        self.0.clear();
      }

      pub fn insert(&mut self, field: impl AsRef<str>) {
        self.0.insert(field.as_ref().to_owned());
      }

      pub fn is_empty(&self) -> bool {
        self.0.is_empty()
      }

      pub fn into_vec(self) -> Vec<String> {
        self.0.into_iter().collect()
      }

      pub fn len(&self) -> usize {
        self.0.len()
      }

      pub fn remove(&mut self, field: impl AsRef<str>) {
        self.0.retain(|f| f != field.as_ref());
      }

      pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
      }

      pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
      }
    }

    impl From<&str> for $target {
      fn from(id: &str) -> Self {
        Self::from(id.to_owned())
      }
    }

    impl From<String> for $target {
      fn from(id: String) -> Self {
        use ahash::HashSet;
        Self(HashSet::from_iter([id]))
      }
    }

    impl From<&String> for $target {
      fn from(id: &String) -> Self {
        Self::from(id.to_owned())
      }
    }

    impl From<std::borrow::Cow<'_, str>> for $target {
      fn from(id: std::borrow::Cow<'_, str>) -> Self {
        Self::from(id.into_owned())
      }
    }

    impl From<&[&str]> for $target {
      fn from(ids: &[&str]) -> Self {
        Self(ids.iter().map(|id| id.to_string()).collect())
      }
    }

    impl<const N: usize> From<[&str; N]> for $target {
      fn from(ids: [&str; N]) -> Self {
        Self::from(ids.as_slice())
      }
    }

    impl<const N: usize> From<&[&str; N]> for $target {
      fn from(ids: &[&str; N]) -> Self {
        Self::from(ids.as_slice())
      }
    }

    impl From<Vec<String>> for $target {
      fn from(ids: Vec<String>) -> Self {
        Self(ids.into_iter().collect())
      }
    }

    impl From<std::collections::HashSet<String>> for $target {
      fn from(ids: std::collections::HashSet<String>) -> Self {
        Self(ids.into_iter().collect())
      }
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_string_set_from_numeric {
  ($prefix:expr, $target:ident) => {
    $crate::impl_string_set_from_numeric!(@ $prefix, i32 => $target);
    $crate::impl_string_set_from_numeric!(@ $prefix, u8 => $target);
    $crate::impl_string_set_from_numeric!(@ $prefix, u16 => $target);
    $crate::impl_string_set_from_numeric!(@ $prefix, u32 => $target);
    $crate::impl_string_set_from_numeric!(@ $prefix, u64 => $target);
  };
  (@ $prefix:expr, $num:ident => $target:ident) => {
    impl From<$num> for $target {
      fn from(id: $num) -> Self {
        Self::from([id])
      }
    }

    impl From<&[$num]> for $target {
      fn from(ids: &[$num]) -> Self {
        Self(ids.iter().map(|id| format!("{}{}", $prefix, id)).collect())
      }
    }

    impl<const N: usize> From<[$num; N]> for $target {
      fn from(ids: [$num; N]) -> Self {
        Self::from(ids.as_slice())
      }
    }

    impl From<Vec<$num>> for $target {
      fn from(ids: Vec<$num>) -> Self {
        Self::from(ids.as_slice())
      }
    }

    impl From<std::ops::Range<$num>> for $target {
      fn from(range: std::ops::Range<$num>) -> Self {
        let numbers = range.collect::<Vec<_>>();
        Self::from(numbers.as_slice())
      }
    }

    impl From<std::ops::RangeInclusive<$num>> for $target {
      fn from(range: std::ops::RangeInclusive<$num>) -> Self {
        let numbers = range.collect::<Vec<_>>();
        Self::from(numbers.as_slice())
      }
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_string_set_from_newtype {
  ($target:ident, $inner:ident) => {
    impl From<$inner> for $target {
      fn from(id: $inner) -> Self {
        Self::from(id.0)
      }
    }

    impl From<&$inner> for $target {
      fn from(id: &$inner) -> Self {
        Self::from(&id.0)
      }
    }

    impl From<Vec<$inner>> for $target {
      fn from(ids: Vec<$inner>) -> Self {
        Self(ids.into_iter().map(|id| id.0).collect())
      }
    }

    impl From<&[$inner]> for $target {
      fn from(ids: &[$inner]) -> Self {
        Self(ids.iter().map(|id| id.0.clone()).collect())
      }
    }

    impl<const N: usize> From<[$inner; N]> for $target {
      fn from(ids: [$inner; N]) -> Self {
        Self::from(ids.as_slice())
      }
    }
  };
}
