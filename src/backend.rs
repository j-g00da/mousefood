use alloc::boxed::Box;
use core::marker::PhantomData;

use crate::colors::*;
use crate::default_font;
use crate::framebuffer;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{self, Dimensions};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::text::Text;
#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use ratatui_core::backend::{Backend, ClearType};
use ratatui_core::layout;
use ratatui_core::style;

/// Embedded backend configuration.
pub struct EmbeddedBackendConfig<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Callback fired after each buffer flush.
    pub flush_callback: Box<dyn FnMut(&mut D)>,
    /// Regular font.
    pub font_regular: MonoFont<'static>,
    /// Bold font.
    pub font_bold: Option<MonoFont<'static>>,
    /// Italic font.
    pub font_italic: Option<MonoFont<'static>>,
}

impl<D, C> Default for EmbeddedBackendConfig<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            flush_callback: Box::new(|_| {}),
            font_regular: default_font::regular,
            font_bold: None,
            font_italic: None,
        }
    }
}

/// Embedded backend for Ratatui.
///
/// # Examples
///
/// ```rust,no_run
/// use mousefood::prelude::*;
///
/// let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
/// let mut terminal = Terminal::new(backend).unwrap();
/// ```
pub struct EmbeddedBackend<'display, D, C>
where
    D: DrawTarget<Color = C> + 'display,
    C: PixelColor + 'display,
{
    #[cfg(not(feature = "simulator"))]
    display: &'display mut D,
    #[cfg(feature = "simulator")]
    display: &'display mut SimulatorDisplay<C>,
    display_type: PhantomData<D>,

    #[cfg(not(feature = "simulator"))]
    flush_callback: Box<dyn FnMut(&mut D)>,
    #[cfg(feature = "simulator")]
    flush_callback: Box<dyn FnMut(&mut SimulatorDisplay<C>)>,

    buffer: framebuffer::HeapBuffer<C>,

    font_regular: MonoFont<'static>,
    font_bold: Option<MonoFont<'static>>,
    font_italic: Option<MonoFont<'static>>,

    char_offset: geometry::Point,

    columns_rows: layout::Size,
    pixels: layout::Size,

    #[cfg(feature = "simulator")]
    simulator_window: Window,
}

impl<'display, D, C> EmbeddedBackend<'display, D, C>
where
    D: DrawTarget<Color = C> + Dimensions + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + From<TermColor> + 'static,
{
    fn init(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
        #[cfg(not(feature = "simulator"))] flush_callback: impl FnMut(&mut D) + 'static,
        #[cfg(feature = "simulator")] flush_callback: impl FnMut(&mut SimulatorDisplay<C>) + 'static,
        font_regular: MonoFont<'static>,
        font_bold: Option<MonoFont<'static>>,
        font_italic: Option<MonoFont<'static>>,
    ) -> EmbeddedBackend<'display, D, C> {
        let pixels = layout::Size {
            width: display.bounding_box().size.width as u16,
            height: display.bounding_box().size.height as u16,
        };
        Self {
            buffer: framebuffer::HeapBuffer::new(display.bounding_box()),
            display,
            display_type: PhantomData,
            flush_callback: Box::new(flush_callback),
            font_regular,
            font_bold,
            font_italic,
            char_offset: geometry::Point::new(0, font_regular.character_size.height as i32),
            columns_rows: layout::Size {
                height: pixels.height / font_regular.character_size.height as u16,
                width: pixels.width / font_regular.character_size.width as u16,
            },
            pixels,
            #[cfg(feature = "simulator")]
            simulator_window: Window::new(
                "mousefood emulator",
                &OutputSettings {
                    scale: 4,
                    max_fps: 30,
                    ..Default::default()
                },
            ),
        }
    }

    /// Creates a new `EmbeddedBackend` using default fonts.
    pub fn new(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
        #[cfg(not(feature = "simulator"))] config: EmbeddedBackendConfig<D, C>,
        #[cfg(feature = "simulator")] config: EmbeddedBackendConfig<SimulatorDisplay<C>, C>,
    ) -> EmbeddedBackend<'display, D, C> {
        Self::init(
            display,
            config.flush_callback,
            config.font_regular,
            config.font_bold,
            config.font_italic,
        )
    }

    #[cfg(feature = "simulator")]
    fn update_simulation(&mut self) -> Result<()> {
        self.simulator_window.update(self.display);
        if self
            .simulator_window
            .events()
            .any(|e| e == SimulatorEvent::Quit)
        {
            return Err(crate::error::Error::SimulatorQuit);
        }
        Ok(())
    }
}

type Result<T, E = crate::error::Error> = core::result::Result<T, E>;

impl<D, C> Backend for EmbeddedBackend<'_, D, C>
where
    D: DrawTarget<Color = C> + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + From<TermColor> + 'static,
{
    type Error = crate::error::Error;

    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui_core::buffer::Cell)>,
    {
        for (x, y, cell) in content {
            let position = geometry::Point::new(
                x as i32 * self.font_regular.character_size.width as i32,
                y as i32 * self.font_regular.character_size.height as i32,
            );

            let mut style_builder = MonoTextStyleBuilder::new()
                .font(&self.font_regular)
                .text_color(TermColor(cell.fg, TermColorType::Foreground).into())
                .background_color(TermColor(cell.bg, TermColorType::Background).into());

            for modifier in cell.modifier.iter() {
                style_builder = match modifier {
                    style::Modifier::BOLD => match &self.font_bold {
                        None => style_builder.font(&self.font_regular),
                        Some(font) => style_builder.font(font),
                    },
                    style::Modifier::DIM => style_builder, // TODO
                    style::Modifier::ITALIC => match &self.font_italic {
                        None => style_builder.font(&self.font_regular),
                        Some(font) => style_builder.font(font),
                    },
                    style::Modifier::UNDERLINED => style_builder.underline(),
                    style::Modifier::SLOW_BLINK => style_builder, // TODO
                    style::Modifier::RAPID_BLINK => style_builder, // TODO
                    style::Modifier::REVERSED => style_builder,   // TODO
                    style::Modifier::HIDDEN => style_builder,     // TODO
                    style::Modifier::CROSSED_OUT => style_builder.strikethrough(),
                    _ => style_builder,
                }
            }

            if cell.underline_color != style::Color::Reset {
                style_builder = style_builder.underline_with_color(
                    TermColor(cell.underline_color, TermColorType::Foreground).into(),
                );
            }

            Text::new(
                cell.symbol(),
                position + self.char_offset,
                style_builder.build(),
            )
            .draw(&mut self.buffer)
            .map_err(|_| crate::error::Error::DrawError)?;
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<layout::Position> {
        // TODO
        Ok(layout::Position::new(0, 0))
    }

    fn set_cursor_position<P: Into<layout::Position>>(
        &mut self,
        #[allow(unused_variables)] position: P,
    ) -> Result<()> {
        // TODO
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.buffer
            .clear(TermColor(style::Color::Reset, TermColorType::Background).into())
            .map_err(|_| crate::error::Error::DrawError)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> Result<()> {
        match clear_type {
            ClearType::All => self.clear(),
            ClearType::AfterCursor
            | ClearType::BeforeCursor
            | ClearType::CurrentLine
            | ClearType::UntilNewLine => Err(crate::error::Error::ClearTypeUnsupported(
                alloc::format!("{:?}", clear_type),
            )),
        }
    }

    fn size(&self) -> Result<layout::Size> {
        Ok(self.columns_rows)
    }

    fn window_size(&mut self) -> Result<ratatui_core::backend::WindowSize> {
        Ok(ratatui_core::backend::WindowSize {
            columns_rows: self.columns_rows,
            pixels: self.pixels,
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.display
            .fill_contiguous(&self.display.bounding_box(), &self.buffer)
            .map_err(|_| crate::error::Error::DrawError)?;
        (self.flush_callback)(self.display);
        #[cfg(feature = "simulator")]
        self.update_simulation()?;
        Ok(())
    }
}
