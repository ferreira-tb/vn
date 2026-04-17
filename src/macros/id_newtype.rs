#[doc(hidden)]
#[macro_export]
macro_rules! impl_id_newtype {
  ($kind:ident, $id:ident, $regex:expr) => {
    impl $id {
      pub fn new(id: impl AsRef<str>) -> Option<Self> {
        let id = id.as_ref();
        if $regex.is_match(id) {
          Some(Self(std::sync::Arc::from(id)))
        } else {
          None
        }
      }

      /// # Safety
      ///
      /// Calling this function with an invalid id is undefined behavior.
      pub unsafe fn new_unchecked(id: impl AsRef<str>) -> Self {
        unsafe { Self::new(id).unwrap_unchecked() }
      }

      pub fn with_suffix(suffix: impl std::fmt::Display) -> Option<Self> {
        Self::new(format!("{}{}", Self::PREFIX, suffix))
      }

      /// # Safety
      ///
      /// Calling this function with an invalid suffix is undefined behavior.
      pub unsafe fn with_suffix_unchecked(suffix: impl std::fmt::Display) -> Self {
        unsafe { Self::with_suffix(suffix).unwrap_unchecked() }
      }

      pub fn from_url(url: &url::Url) -> Option<Self> {
        if url.host_str()?.contains("vndb.org") {
          url.path_segments()?.find_map(Self::new)
        } else {
          None
        }
      }

      /// # Safety
      ///
      /// Calling this function with a URL that doesn't contain a valid id is undefined behavior.
      pub unsafe fn from_url_unchecked(url: &url::Url) -> Self {
        unsafe { Self::from_url(url).unwrap_unchecked() }
      }

      pub fn regex() -> &'static regex::Regex {
        &$regex
      }
    }

    impl Clone for $id {
      fn clone(&self) -> Self {
        Self(std::sync::Arc::clone(&self.0))
      }
    }

    impl PartialEq<str> for $id {
      fn eq(&self, other: &str) -> bool {
        self.0.as_ref() == other
      }
    }

    impl std::ops::Deref for $id {
      type Target = str;

      fn deref(&self) -> &Self::Target {
        self.0.as_ref()
      }
    }

    impl std::str::FromStr for $id {
      type Err = $crate::error::Error;

      fn from_str(id: &str) -> Result<Self, Self::Err> {
        Self::new(id).ok_or_else(|| $crate::error::Error::InvalidId(id.to_owned()))
      }
    }

    impl From<$id> for $crate::model::VndbId {
      fn from(id: $id) -> Self {
        $crate::model::VndbId::$kind(id)
      }
    }

    #[cfg(feature = "diesel_sqlite")]
    impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for $id {
      fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>,
      ) -> diesel::deserialize::Result<Self> {
        let value = <String as diesel::deserialize::FromSql<
          diesel::sql_types::Text,
          diesel::sqlite::Sqlite,
        >>::from_sql(bytes)?;

        Ok(value.as_str().parse()?)
      }
    }

    #[cfg(feature = "diesel_sqlite")]
    impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for $id
    where
      String: diesel::serialize::ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite>,
    {
      fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
      ) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
      }
    }
  };
}
