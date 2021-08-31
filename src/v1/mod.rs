//! [Giphy API v1] wrapper for Rust
//!
//! [Giphy API v1]: https://developers.giphy.com/docs/api/
//! [`search`]: https://developers.giphy.com/docs/api/endpoint/#search
pub mod r#async;
pub mod gifs;
mod model;
pub mod sync;

pub use model::*;
