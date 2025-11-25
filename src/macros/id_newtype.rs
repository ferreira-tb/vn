#[doc(hidden)]
#[macro_export]
macro_rules! impl_id_newtype_regex {
  ($target:ident, $regex:expr) => {
    impl $target {
      pub fn regex() -> &'static regex::Regex {
        &$regex
      }

      pub fn parse(value: impl AsRef<str>) -> $crate::error::Result<Self> {
        let id = Self(value.as_ref().to_owned());
        id.validate().map(|()| id)
      }

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
