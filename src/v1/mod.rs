//! [Giphy API v1] wrapper for Rust
//!
//! [Giphy API v1]: https://developers.giphy.com/docs/
//! [`search`]: https://developers.giphy.com/docs/#path--gifs-search
pub mod r#async;
pub mod gifs;
mod model;
pub mod sync;

pub use model::*;
