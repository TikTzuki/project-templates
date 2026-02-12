//! Core library for the workspace.

mod error;

pub use error::AppError;

/// Returns the crate version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
