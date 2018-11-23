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

#[cfg_attr(feature = "protocol", macro_use)]
#[cfg(feature = "protocol")]
extern crate protocol_derive;

mod bindings;

pub use self::bindings::*;

