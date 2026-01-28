//! A prelude for conveniently writing applications using this library.
//!
//! # Examples
//!
//! ```rust
//! use mousefood::embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};
//! use mousefood::prelude::*;
//!
//! let mut display = MockDisplay::<Rgb888>::new();
//! let _backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
//! ```

pub use crate::ColorTheme;
pub use crate::backend::{EmbeddedBackend, EmbeddedBackendConfig};
pub use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, Rgb555, Rgb565, Rgb666, Rgb888,
};
