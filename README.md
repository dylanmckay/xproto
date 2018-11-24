# xproto

[Documentation](https://docs.rs/xproto)

Bindings to the [xproto library](https://gitlab.freedesktop.org/xorg/proto/xproto).

Rust transpilation of the `xproto` library.

This library exposes types and values required to communicate with, or as, an X11 server.

The C `xproto` library does not contain any logic - only datastructures and constants.

This means the library does not contain any C code or link to any non-Rust object files.

