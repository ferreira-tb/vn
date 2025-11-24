#[doc(hidden)]
#[macro_export]
macro_rules! impl_id_newtype {
  ($target:ident) => {
    impl $target {
      pub fn into_inner(self) -> String {
        self.0
      }
    }

    impl From<&str> for $target {
      fn from(id: &str) -> Self {
        Self(id.to_owned())
      }
    }

    impl From<String> for $target {
      fn from(id: String) -> Self {
        Self(id)
      }
    }

    impl From<&String> for $target {
      fn from(id: &String) -> Self {
        Self(id.to_owned())
      }
    }

    impl From<std::borrow::Cow<'_, str>> for $target {
      fn from(id: std::borrow::Cow<'_, str>) -> Self {
        Self(id.into_owned())
      }
    }

    impl std::ops::Deref for $target {
      type Target = str;

      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_id_newtype_regex {
  ($target:ident, $regex:expr) => {
    impl $target {
      #[cfg(feature = "regex")]
      pub fn regex() -> &'static regex::Regex {
        &$regex
      }

      #[cfg(feature = "regex")]
      pub fn parse(value: impl AsRef<str>) -> $crate::error::Result<Self> {
        let id = Self(value.as_ref().to_owned());
        id.validate().map(|()| id)
      }

      #[cfg(feature = "regex")]
      pub fn validate(&self) -> $crate::error::Result<()> {
        if $regex.is_match(&self.0) {
          Ok(())
        } else {
          Err($crate::error::Error::InvalidId(self.0.clone()))
        }
      }
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_id_newtype_from_numeric {
  ($prefix:expr, $target:ident) => {
    $crate::impl_id_newtype_from_numeric!(@ $prefix, i8 => $target);
    $crate::impl_id_newtype_from_numeric!(@ $prefix, i16 => $target);
    $crate::impl_id_newtype_from_numeric!(@ $prefix, i32 => $target);
    $crate::impl_id_newtype_from_numeric!(@ $prefix, i64 => $target);
    $crate::impl_id_newtype_from_numeric!(@ $prefix, u8 => $target);
    $crate::impl_id_newtype_from_numeric!(@ $prefix, u16 => $target);
    $crate::impl_id_newtype_from_numeric!(@ $prefix, u32 => $target);
    $crate::impl_id_newtype_from_numeric!(@ $prefix, u64 => $target);
  };
  (@ $prefix:expr, $num:ident => $target:ident) => {
    impl From<$num> for $target {
      fn from(id: $num) -> Self {
        Self(format!("{}{}", $prefix, id))
      }
    }
  };
}
