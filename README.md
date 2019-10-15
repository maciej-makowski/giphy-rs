# [Giphy] library for Rust

[![](https://meritbadge.herokuapp.com/giphy)](https://crates.io/crates/giphy)
[![Build Status](https://travis-ci.org/cfiet/giphy-rs.svg?branch=master)](https://travis-ci.org/cfiet/giphy-rs)

This library provides Rust wrapper for [Giphy HTTP API]

## Features

Version `0.3.0` is feature complete and allows to search and retrieve GIFs from [Giphy] both in
synchronous and asynchronous style. See [Crate documentation] and examples for details.
Stickers API and posting GIFs is currently not supported. 

This library is still under development and the API is subject to change. Since [Giphy] does not specify the 
parts of GIF object that are optional, there is potentially still an issue with response object model,
where actual objects returned by API may not be possible to deserialize into Rust. I am making an
active effort to integration-test the library an make sure that the response model is complete,
but if you run into an JSON deserialization error, create an issue with the object ID / request params
and I will patch the response model.

## Usage and examples
See [Crate documentation] and [examples]

## Documentation
 - [Crate documentation] - API reference and example usage
 - [Giphy HTTP API] docs

[Crate documentation]: https://docs.rs/giphy
[examples]: ./examples
[Giphy]: https://giphy.com/
[Giphy HTTP API]: https://developers.giphy.com/docs
