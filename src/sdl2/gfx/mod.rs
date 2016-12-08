//!
//! A binding for the library `SDL2_gfx`
//!
//! 
//! Note that you need to build with the
//! feature `gfx` for this module to be enabled,
//! like so:
//!
//! ```bash
//! $ cargo build --features "gfx"
//! ```
//!
//! If you want to use this with from inside your own
//! crate, you will need to add this in your Cargo.toml
//!
//! ```toml
//! [dependencies.sdl2]
//! version = ...
//! default-features = false
//! features = ["gfx"]
//! ```
extern crate c_vec;

// Setup linking for all targets.
#[link(name="SDL2_gfx")]
extern {}

pub mod primitives;
pub mod rotozoom;
pub mod framerate;
pub mod imagefilter;
