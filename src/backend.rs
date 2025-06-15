use core::marker::PhantomData;
use std::io;

use crate::colors::*;
use crate::default_font;
use crate::error::DrawError;
use crate::framebuffer;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{self, Dimensions};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::text::Text;
#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use ratatui::backend::Backend;
use ratatui::layout;
use ratatui::style;

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
    fn update_simulation(&mut self) -> io::Result<()> {
        self.simulator_window.update(self.display);
        if self
            .simulator_window
            .events()
            .any(|e| e == SimulatorEvent::Quit)
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "simulator window closed",
            ));
        }
        Ok(())
    }
}

impl<D, C> Backend for EmbeddedBackend<'_, D, C>
where
    D: DrawTarget<Color = C> + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + From<TermColor> + 'static,
{
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>,
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

            Text::new(
                cell.symbol(),
                position + self.char_offset,
                style_builder.build(),
            )
            .draw(&mut self.buffer)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, DrawError))?;
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        // TODO
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        // TODO
        Ok(())
    }

    fn get_cursor_position(&mut self) -> io::Result<layout::Position> {
        // TODO
        Ok(layout::Position::new(0, 0))
    }

    fn set_cursor_position<P: Into<layout::Position>>(
        &mut self,
        #[allow(unused_variables)] position: P,
    ) -> io::Result<()> {
        // TODO
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.buffer
            .clear(TermColor(ratatui::style::Color::Reset, TermColorType::Background).into())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, DrawError))
    }

    fn size(&self) -> io::Result<layout::Size> {
        Ok(self.columns_rows)
    }

    fn window_size(&mut self) -> io::Result<ratatui::backend::WindowSize> {
        Ok(ratatui::backend::WindowSize {
            columns_rows: self.columns_rows,
            pixels: self.pixels,
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.display
            .fill_contiguous(&self.display.bounding_box(), &self.buffer)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, DrawError))?;
        (self.flush_callback)(self.display);
        #[cfg(feature = "simulator")]
        self.update_simulation()?;
        Ok(())
    }
}
