//! [Giphy API v1] wrapper for Rust
//!
//! [Giphy API v1]: https://developers.giphy.com/docs/api/
//! [`search`]: https://developers.giphy.com/docs/api/endpoint/#search


pub mod gifs;
mod model;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod r#async;

pub use model::*;
