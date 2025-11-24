$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

cargo clippy --workspace
cargo test --tests -- --test-threads=1
