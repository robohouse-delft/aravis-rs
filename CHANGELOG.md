v0.11.0 - 2024-09-09:
  * Update `image` crate to `v0.25`.

v0.10.0 - 2024-08-01:
  * Add support for new features from Aravis `v0.8.31`.
  * The `Stream::push_buffer()` function now takes the buffer by value.
  * Remove auto-generated `Display` implementations with practically no information.

v0.9.2 - 2024-06-28:
  * Make the `image` crate an optional dependency (enabled by default).
  * Add `Buffer::into_boxed_slice()`.

v0.9.1 - 2023-08-29
  * Add `GcRegisterExtManual` with `get_raw()` and `set_raw()`.

v0.9.0 - 2023-05-07:
  * Correctly forward `aravis-sys` features in `aravis`.
  * Put `--usb-mode` flag behind `usb-mode` features for compatibility with older Aravis versions.
  * Replace `structopt` with `clap` for command line utilities.
  * Update `glib` related dependencies.

v0.8.3 - 2023-02-24:
  * Add `--usb-mode` option to `aravis-capture`.
  * Expose features from `aravis-sys` in `aravis`.

v0.8.2 - 2023-01-02:
  * Add support for Aravis `v0.8.22`.

v0.8.1 - 2022-11-30:
  * Add `Buffer::new_preallocated_owned(...)` to transfer ownership of a pre-allocated buffer to Aravis.
  * Deprecate `Buffer::new_preallocated(...)` in favor of `Buffer::new_preallocated_borrowed`.

v0.8.0:
  * Update to `gtk-rs` v0.15.
  * Update to `image` v0.24.
  * Remove support for converting BGR images to `image::DynamicImage`.

v0.7.1:
  * Fix documentation generation for `aravis` crate.

v0.7.0:
  * Generate safe API with latest `gir` tool (see library docs for more info on backwards incompatible changes).
  * Update to show-image 0.9 with zoom/pan support.
  * Remove debug print from debayering code.

v0.6.0:
  * Update to Aravis 0.8.5.

v0.5.0:
  * Disable "bayer" feature by default.
  * Fix use of `MaybeUnit` for latest nightly.
  * Add fallbacks for unstable features so the crate work on stable.

v0.4.1:
  * Fix formatting of docmentation.

v0.4.0:
  * Update to Aravis 0.8.1.

v0.3.2:
  * Update dependency version.

v0.3.1:
  * Make Aravis types impl `Send`.

v0.3.0:
  * Use a newtype wrapper for `PixelFormat` with associated constants.
  * Target officially released Aravis 0.7.5.

v0.2.1:
  * Target Aravis 0.7.5.

aravis v0.2.0 and aravis-utils v0.2.0:
  * Update to Aravis v0.7.99 (current git master)

aravis v0.1.0 and aravis-utils v0.1.2:
  * Update dependencies.
  * Fix clippy warnings.
