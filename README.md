# [Giphy] library for Rust

[![crates.io](https://img.shields.io/crates/v/giphy.svg)](https://crates.io/crates/giphy)
[![Build Status](https://travis-ci.com/maciej-makowski/giphy-rs.svg?branch=master)](https://travis-ci.com/cfiet/giphy-rs)

This library provides Rust wrapper for [Giphy HTTP API]

## API Features
Version `0.4.0` is feature complete and allows to search and retrieve GIFs from [Giphy] both in
synchronous and asynchronous style. See [Crate documentation] and examples for details.
Stickers API and posting GIFs is currently not supported. 

This library is still under development and the API is subject to change. Since [Giphy] does not specify
optional parts of GIF object, there might still be an issue with response model where objects returned by
API may not be deserialised. I am making an active effort to integration-test the library to make sure
the response model is complete. If you run into an JSON deserialization error, please create an issue with
the object ID / request params and I will patch the response model.

## Features
By default both `sync` and `async` API dependencies are included with the library. If you do not
need and `async` support, you can reduce the dependency size and build time by only loading `feature = ["sync"]`.

## Usage and examples
See [Crate documentation] and [examples]

## Documentation
 - [Crate documentation] - API reference and example usage
 - [Giphy HTTP API] docs

[Crate documentation]: https://docs.rs/giphy
[examples]: ./examples
[Giphy]: https://giphy.com/
[Giphy HTTP API]: https://developers.giphy.com/docs/api/
