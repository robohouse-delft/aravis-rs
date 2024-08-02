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

## How to update Aravis API?

1. Build Aravis itself from [source](https://github.com/AravisProject/aravis) and copy the generated Aravis-*.gir (inside src) to the gir-files folder.

2. Install Aravis so that the build script for the FFI crate can find it. You should be able to install it in a user directory, as long as pkgconf/pkg-config can find it.

3. If you want to update the gtk-rs gir files, you should just update the submodule to the current master. The relevant files are symlinked, so you don't have to copy anything manually.

4. Run the gir tool from gtk-rs in the root of the aravis-sys crate. All required options are taken from the Gir.toml file. Documentation related to gir tool can be found [here](https://gtk-rs.org/gir/book/).

5. Run the gir tool in the aravis crate. This time there should be no manual intervention needed. Some of the items might not be generated which can be fixed by following the [manual](https://gtk-rs.org/gir/book/tutorial/high_level_rust_api.html).

6. Run cargo fmt in both crates after re-generating the files.

7. Run gir -m doc in each crate followed by rustdoc-stripper -g -o vendor.md. You can install rustdoc-stripper from https://crates.io. Don't commit the `vendor.md`.

8. Run cargo fmt again after.
