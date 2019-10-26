This repository contains Rust bindings for [Aravis](https://github.com/AravisProject/aravis), a vision library for genicam based cameras.

**Warnings:**
These bindings are still very unstable, and they have not been tested yet.
Some parts of the Aravis API may be missing, and some may be removed in the future if they don't make sense for Rust.

This repository contains two crates:
* `aravis`: Safe wrappers around `aravis-ffi`.
* `aravis-ffi`: Raw FFI bindings to Aravis.

Both crates are entirely or mostly generated from the relevant [GIR](https://gi.readthedocs.io/en/latest/) files,
with the [`gir`](https://github.com/gtk-rs/gir) tool from the [`gtk-rs`](https://github.com/gtk-rs) project
