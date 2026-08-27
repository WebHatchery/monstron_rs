//! Compile-time identity for player reports and release artifacts.

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_ID: &str = env!("HATCHSPIRE_BUILD_ID");
pub const TOOLKIT_BUILD_ID: &str = env!("HATCHSPIRE_TOOLKIT_BUILD_ID");

#[cfg(test)]
mod tests;
