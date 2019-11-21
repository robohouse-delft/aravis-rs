This repository contains Rust bindings for [Aravis](https://github.com/AravisProject/aravis), a vision library for GenICam based cameras.

This repository contains three crates:
* `aravis`: Safe (mostly) wrappers around `aravis-ffi`.
* `aravis-utils`: Command line utilities using the `aravis` crate.
* `aravis-ffi`: Raw FFI bindings to Aravis.

The API binding crates are largely generated from the relevant [GIR](https://gi.readthedocs.io/en/latest/) files,
with the [`gir`](https://github.com/gtk-rs/gir) tool from the [`gtk-rs`](https://github.com/gtk-rs) project

**Warnings:**
These bindings are still very unstable, and they have not all been tested yet.
Some parts of the Aravis API may be missing, and some may be removed in the future if they don't make sense for Rust.
As the `aravis-utils` crate is extended with more full-featured utilities, more parts of the API will receive testing.
