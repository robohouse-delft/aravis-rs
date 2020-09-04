//! This crate contains (mostly) safe bindings to the [Aravis][aravis] library.
//! The bindings are mostly auto-generated with the [`gir`][gir-tool] tool from the [gtk-rs][gtk-rs] project.
//!
//! This crate currently targets version 0.8.1 of the Aravis library.
//!
//! This documentation constist mainly of original documentation of the Aravis project.
//! The copyright and license of the Aravis project apply to those parts.
//! The [full original documentation][aravis-docs] is also available online and might help if the translation to Rust made things unclear.
//!
//! [aravis]: https://github.com/AravisProject/aravis
//! [aravis-docs]: https://aravisproject.github.io/docs/aravis-0.8/
//! [gir-tool]: https://github.com/gtk-rs/gir
//! [gtk-rs]: https://gtk-rs.org/

#![feature(new_uninit)]
#![feature(maybe_uninit_slice)]

extern crate aravis_sys;
extern crate gio;
extern crate gio_sys;
#[macro_use]
pub extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate image;

macro_rules! assert_initialized_main_thread {
	() => {};
}

/// No-op.
macro_rules! skip_assert_initialized {
	() => {};
}

#[allow(clippy::all)]
mod auto;
pub use auto::*;

mod manual;
pub use manual::*;

pub mod prelude {
	pub use auto::traits::*;
	pub use manual::traits::*;
}
