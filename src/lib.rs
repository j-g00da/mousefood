#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod backend;
mod colors;
mod default_font;
pub mod error;
mod framebuffer;
mod macros;
pub mod prelude;

pub use backend::{EmbeddedBackend, EmbeddedBackendConfig};
pub use embedded_graphics;

#[cfg(feature = "simulator")]
pub use embedded_graphics_simulator as simulator;

#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts as fonts;
