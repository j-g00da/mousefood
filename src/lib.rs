#![doc = include_str!("../README.md")]

extern crate alloc;

mod backend;
mod colors;
mod default_font;
mod error;
mod framebuffer;
mod macros;
pub mod prelude;

pub use backend::{EmbeddedBackend, EmbeddedBackendConfig};
pub use embedded_graphics;
pub use ratatui;

#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts as fonts;
