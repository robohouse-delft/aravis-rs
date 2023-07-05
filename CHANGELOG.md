Unreleased
  * Add `get` and `set` functions to GcRegisterExt.

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
