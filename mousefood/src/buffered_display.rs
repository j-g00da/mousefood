use crate::colors::TermColor;
pub use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::PixelColor;

/// Flushing to the display failed.
#[derive(thiserror::Error, Debug)]
#[error("flushing to DrawTarget failed: {0}")]
pub struct FlushError(pub alloc::string::String);

/// Trait to implement for displays to be usable with mousefood.
///
/// The display must implement [`DrawTarget`]
pub trait BufferedDisplay<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + From<TermColor>,
{
    /// Get the place where new pixels will be drawn to before being shown on the display.
    ///
    /// Many displays backends have a built-in buffer, but some do not.
    /// An external display buffer can be provided.
    fn draw_target(&mut self) -> &mut impl DrawTarget<Color = C>;

    /// Display contents of the in-memory display to the screen
    ///
    /// If the display driver requires additional operations, this is the place to make them
    fn flush(&mut self) -> Result<(), FlushError>;
}
