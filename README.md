This repository contains Rust bindings for [Aravis](https://github.com/AravisProject/aravis), a vision library for GenICam based cameras.

This repository contains three crates:
* `aravis`: Safe wrappers around `aravis-sys`.
* `aravis-utils`: Command line utilities using the `aravis` crate.
* `aravis-sys`: Raw FFI bindings to Aravis.

The API binding crates are largely generated from the relevant [GIR](https://gi.readthedocs.io/en/latest/) files,
with the [`gir`](https://github.com/gtk-rs/gir) tool from the [`gtk-rs`](https://github.com/gtk-rs) project

**Note:**
Not all of the API has been tested.
Some parts of the Aravis API may be missing,
and some may be removed in the future if they don't make sense for Rust.
