//! [Giphy API] wrapper for Rust
//!
//! The library provides both synchronous and asynchronous API wrappers. It uses [`reqwest`] crate
//! for making the HTTP requests.
//!
//! See [`v1`] module for Giphy v1 API wrapper
//!
//! [Giphy API]: https://developers.giphy.com/docs/
//! [`reqwest`]: https://docs.rs/reqwest
//! [`v1`]: ./v1/index.html

extern crate futures;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate dotenv;
#[cfg(test)]
extern crate tokio;

pub mod v1;
