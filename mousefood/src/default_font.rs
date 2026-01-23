#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts::mono_6x10_atlas as get_regular;

#[cfg(not(feature = "fonts"))]
pub fn get_regular() -> embedded_graphics::mono_font::MonoFont<'static> {
    embedded_graphics::mono_font::ascii::FONT_6X10
}
