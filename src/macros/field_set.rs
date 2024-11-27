#[doc(hidden)]
#[macro_export]
macro_rules! impl_into_field_set {
  ($target:ident) => {
    impl $target {
      pub fn all() -> $crate::http::FieldSet<$target> {
        $crate::http::FieldSet::from($target::VARIANTS)
      }

      pub fn none() -> $crate::http::FieldSet<$target> {
        $crate::http::FieldSet::default()
      }

      pub fn into_field_set(self) -> $crate::http::FieldSet<$target> {
        $crate::http::FieldSet::from(self)
      }
    }

    impl From<$target> for $crate::http::FieldSet<$target> {
      fn from(field: $target) -> Self {
        let mut set = Self::with_capacity(1);
        set.insert(&field);
        set
      }
    }

    impl From<&[$target]> for $crate::http::FieldSet<$target> {
      fn from(fields: &[$target]) -> Self {
        let mut set = Self::with_capacity(fields.len());
        fields.iter().for_each(|field| set.insert(field));
        set
      }
    }

    impl<const N: usize> From<[$target; N]> for $crate::http::FieldSet<$target> {
      fn from(fields: [$target; N]) -> Self {
        Self::from(fields.as_slice())
      }
    }

    impl<const N: usize> From<&[$target; N]> for $crate::http::FieldSet<$target> {
      fn from(fields: &[$target; N]) -> Self {
        Self::from(fields.as_slice())
      }
    }

    impl From<Vec<$target>> for $crate::http::FieldSet<$target> {
      fn from(fields: Vec<$target>) -> Self {
        Self::from(fields.as_slice())
      }
    }
  };
}
