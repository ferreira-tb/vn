use strum::VariantArray;
use vn::http::Endpoint;

#[test]
fn endpoints_are_valid() {
  for endpoint in Endpoint::VARIANTS {
    // This will panic if the URL is invalid.
    endpoint.url();
  }
}
