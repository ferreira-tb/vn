mod field_set;
mod id_newtype;
mod string_set;

#[doc(hidden)]
#[macro_export]
macro_rules! make_request {
  ($vndb:expr, $request:expr) => {{
    $request
      .semaphore(&$vndb.semaphore)
      .maybe_token($vndb.token.as_ref())
      .maybe_timeout($vndb.timeout.clone())
      .maybe_user_agent($vndb.user_agent.as_deref())
      .call()
      .await
  }};
}