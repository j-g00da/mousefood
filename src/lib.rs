#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod backend;
mod colors;
mod default_font;
mod error;
mod framebuffer;
pub mod prelude;

pub use backend::EmbeddedBackend;
pub use embedded_graphics;
pub use ratatui;

#[cfg(feature = "simulator")]
pub use embedded_graphics_simulator as simulator;

#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts as fonts;
