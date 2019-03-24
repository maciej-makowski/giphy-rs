//! [Giphy API v1] wrapper for Rust
//!
//! # Examples
//!
//! ## Synchronous client
//! ```
//! use giphy::v1::{SyncApi, API_ROOT};
//!
//! let key = "Giphy API key".to_string();
//! let client = reqwest::Client::new();
//! let api = SyncApi::new(API_ROOT.to_string(), key, client);
//! ```
//!
//! ### Asynchronous client
//! ```
//! use giphy::v1::{AsyncApi, API_ROOT};
//!
//! let key = "Giphy API key".to_string();
//! let client = reqwest::r#async::Client::new();
//! let api = AsyncApi::new(API_ROOT.to_string(), key, client);
//! ```
//!
//! [Giphy API v1]: https://developers.giphy.com/docs/
mod model;
mod sync;
mod r#async;

pub use model::*;
pub use sync::Api as SyncApi;
pub use r#async::Api as AsyncApi;
