//! # [Giphy] API wrapper for Rust
//!
//! The library provides a wrapper for [Giphy HTTP API].
//! 
//! ## Features
//! Version `0.3.0` is features complete and implements all the methods for searching
//! and retrieving GIFs from [Giphy] using `v1` API. Both synchronous and asynchronous
//! styles are supported by the library.
//! 
//! Stickers and posting GIFs to [Giphy] is currently not supported.
//! 
//! ## Examples
//! 
//! 
//!
//! [Giphy]: https://giphy.com
//! [Giphy API]: https://developers.giphy.com/docs/
//! [Giphy HTTP API]: https://developers.giphy.com/docs/
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
