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
//! ### Synchronous API
//! Create a synchronous [`reqwest::Client`] and [`SyncApi`] object holding your API key
//! 
//! ```
//! use giphy::v1::sync::*;
//! 
//! let client = reqwest::Client::new();
//! let api = SyncApi::new("[your Giphy API key]".to_string(), client);
//! ```
//! 
//! Then create and customise an appropriate request object from [`giphy::v1::gifs`] module
//! to make the request.
//! 
//! For example, to search for gifs related to "tacos"
//! 
//! ```no_run
//! # use giphy::v1::sync::*;
//! use giphy::v1::gifs::SearchRequest;
//! # let client = reqwest::Client::new();
//! # let api = SyncApi::new("[your Giphy API key]".to_string(), client);
//! 
//! let response = SearchRequest::new("tacos")  // Create search request
//!     .with_limit(10)                         // Limit the number of objects in response
//!     .send_to(&api)                          // Send the request using SyncApi
//!     .unwrap_or_else(|e|                     // Unwrap the response Result
//!         panic!("Search request failed: {:?}", e)
//!     );
//! 
//! let first_result = response.data            // response.data will contain GIFs data
//!     .first()
//!     .unwrap_or_else(||
//!         panic!("No results returned from Giphy")
//!     );
//! ```
//! 
//! ### Asynchronous API
//! Create an asynchronous [`reqwest::async::Client`] and [`AsyncApi`] object holding your API key
//! 
//! ```
//! use giphy::v1::r#async::*;
//! 
//! let client = reqwest::r#async::Client::new();
//! let api = AsyncApi::new("[your Giphy API key]".to_string(), client);
//! ```
//! 
//! The request object is created the same way as when using the synchronous API. The
//! only difference is, [`send_to`] method will return a [`Future`] object when called
//! with [`AsyncApi`].
//! 
//! ```no_run
//! use futures::future::Future;
//! # use giphy::v1::r#async::*;
//! use giphy::v1::gifs::SearchRequest;
//! # let client = reqwest::r#async::Client::new();
//! # let api = AsyncApi::new("[your Giphy API key]".to_string(), client);
//! // Create search request the same way as with synchronous api
//! let search_fut = SearchRequest::new("tacos")
//!     .with_limit(10)
//!     .send_to(&api) // Send the request using AsyncApi, this returns a Future
//!     .map(|response| {
//!         // Operate on response
//!         println!("Response: {:?}", response);
//!     });
//! ```
//! The `search_fut` can then be run with [`tokio`]. 
//! 
//! ### More examples
//! See [examples] for showcase of all the currently possible [Giphy HTTP API]
//! requests in both synchronous and asynchronous style.
//! 
//! To run the examples, create a `.env` file in the repository root with the
//! following content
//! ```sh
//! GIPHY_API_KEY_TEST=[your Giphy API key]
//! ```
//!
//! [Giphy]: https://giphy.com
//! [Giphy API]: https://developers.giphy.com/docs/
//! [Giphy HTTP API]: https://developers.giphy.com/docs/
//! [`reqwest`]: https://docs.rs/reqwest
//! [`v1`]: ./v1/index.html
//! [`reqwest::Client`]: ../reqwest/struct.Client.html
//! [`reqwest::async::Client`]: ../reqwest/async/struct.Client.html
//! [`SyncApi`]: v1/sync/struct.SyncApi.html
//! [`AsyncApi`]: v1/async/struct.AsyncApi.html
//! [`giphy::v1::gifs`]: v1/gifs/index.html
//! [examples]: https://github.com/cfiet/giphy-rs/tree/master/examples
//! [`tokio`]: ../tokio/index.html

extern crate futures;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate dotenv;
#[cfg(test)]
extern crate tokio;

pub mod v1;
