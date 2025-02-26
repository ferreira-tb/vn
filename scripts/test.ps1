$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

cargo +nightly clippy
cargo test --tests -- --test-threads=1
