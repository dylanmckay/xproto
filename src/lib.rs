//! Rust transpilation of the `xproto` library.
//!
//! This library exposes types and values required
//! to communicate with, or as, an X11 server.
//!
//! The `xproto` library does not include any logic,
//! only data structures and constants.

// Allow C style.
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate protocol_derive;
extern crate protocol;

mod bindings;
#[cfg(test)] mod tests;

pub use self::bindings::*;

