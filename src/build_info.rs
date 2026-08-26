//! Compile-time identity for player reports and release artifacts.

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_ID: &str = env!("HATCHSPIRE_BUILD_ID");

#[cfg(test)]
mod tests;
