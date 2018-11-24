//! Rust transpilation of the `xproto` library.
//!
//! This library exposes types and values required
//! to communicate with, or as, an X11 server.
//!
//! The C [`xproto` library](https://cgit.freedesktop.org/xorg/proto/xproto/tree)
//! does not contain any logic - only datastructures and constants.
//!
//! This means the library does not contain any C code or link to
//! any non-Rust object files.

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

