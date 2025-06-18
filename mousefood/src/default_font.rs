#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts::MONO_6X10 as regular;

#[cfg(not(feature = "fonts"))]
pub use embedded_graphics::mono_font::ascii::FONT_6X10 as regular;
