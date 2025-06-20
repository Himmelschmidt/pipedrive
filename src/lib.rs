#[cfg(feature = "v1")]
pub mod v1;

#[cfg(feature = "v2")]
pub mod v2;

#[cfg(feature = "v2")]
pub use v2::{apis, models};
