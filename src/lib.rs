pub mod prelude;

pub use embedded_graphics;
pub use ratatui;

use prelude::*;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{self, Dimensions};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{PixelColor, RgbColor};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use embedded_graphics_framebuf::FrameBuf;

use backend::WindowSize;
use buffer::Cell;

use std::io;
use std::marker::PhantomData;

#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

#[cfg(feature = "fonts")]
mod default_font {
    pub use ibm437::IBM437_8X8_BOLD as bold;
    pub use ibm437::IBM437_8X8_REGULAR as regular;
}
#[cfg(not(feature = "fonts"))]
mod default_font {
    pub use embedded_graphics::mono_font::ascii::FONT_4X6 as regular;
    pub use embedded_graphics::mono_font::ascii::FONT_4X6 as bold;
}

enum TermColorType {
    Foreground,
    Background,
}

struct TermColor(Color, TermColorType);

pub struct EmbeddedBackend<'display, D, C, const N: usize>
where
    D: DrawTarget<Color = C> + 'display,
    C: PixelColor + 'display,
{
    #[cfg(not(feature = "simulator"))]
    display: &'display mut D,
    #[cfg(feature = "simulator")]
    display: &'display mut SimulatorDisplay<C>,
    display_area: Rectangle,
    display_type: PhantomData<D>,

    buffer: FrameBuf<C, [C; N]>,

    font_regular: MonoFont<'static>,
    font_bold: MonoFont<'static>,

    char_offset: geometry::Point,

    columns_rows: Size,
    pixels: Size,

    #[cfg(feature = "simulator")]
    simulator_window: Window,
}

impl<'display, D, C, const N: usize> EmbeddedBackend<'display, D, C, N>
where
    D: DrawTarget<Color = C> + Dimensions,
    C: RgbColor + Into<Rgb888> + From<Rgb888>,
{
    pub fn new(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
    ) -> EmbeddedBackend<'display, D, C, N> {
        Self::with_font(display, None, None)
    }

    pub fn with_font(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
        font_regular: Option<MonoFont<'static>>,
        font_bold: Option<MonoFont<'static>>,
    ) -> EmbeddedBackend<'display, D, C, N> {
        let font_regular = font_regular.unwrap_or(default_font::regular);
        let font_bold = font_bold.unwrap_or(default_font::bold);
        Self::init(display, font_regular, font_bold)
    }

    fn init(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
        font_regular: MonoFont<'static>,
        font_bold: MonoFont<'static>,
    ) -> EmbeddedBackend<'display, D, C, N> {
        let display_area = display.bounding_box();
        let pixels = Size {
            width: display_area.size.width as u16,
            height: display_area.size.height as u16,
        };

        Self {
            display,
            display_area,
            display_type: PhantomData,
            buffer: FrameBuf::new(
                [C::BLACK; N],
                display_area.size.width as usize,
                display_area.size.height as usize,
            ),
            font_regular,
            font_bold,
            char_offset: geometry::Point::new(0, font_regular.character_size.height as i32),
            columns_rows: Size {
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

#[derive(Debug)]
struct DrawError;

impl std::error::Error for DrawError {}

impl std::fmt::Display for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "writing to display failed")
    }
}

macro_rules! impl_for_color {
    (
        $color_type:ident
    ) => {
        impl From<TermColor> for $color_type {
            fn from(color: TermColor) -> Self {
                const R_RATIO: f32 = $color_type::MAX_R as f32 / 256.0;
                const G_RATIO: f32 = $color_type::MAX_G as f32 / 256.0;
                const B_RATIO: f32 = $color_type::MAX_B as f32 / 256.0;

                const LIGHT_RED: $color_type = $color_type::new(
                    $color_type::MAX_R,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B / 2,
                );
                const LIGHT_GREEN: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G,
                    $color_type::MAX_B / 2,
                );
                const LIGHT_YELLOW: $color_type = $color_type::new(
                    $color_type::MAX_R,
                    $color_type::MAX_G,
                    $color_type::MAX_B / 2,
                );
                const LIGHT_BLUE: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B,
                );
                const LIGHT_MAGENTA: $color_type = $color_type::new(
                    $color_type::MAX_R,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B,
                );
                const LIGHT_CYAN: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G,
                    $color_type::MAX_B,
                );
                const GRAY: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B / 2,
                );
                const DARK_GRAY: $color_type = $color_type::new(
                    (2.0 / 3.0 * ($color_type::MAX_R as f32)) as u8,
                    (2.0 / 3.0 * ($color_type::MAX_G as f32)) as u8,
                    (2.0 / 3.0 * ($color_type::MAX_B as f32)) as u8,
                );

                match color.0 {
                    Color::Reset => match color.1 {
                        TermColorType::Foreground => $color_type::WHITE,
                        TermColorType::Background => $color_type::BLACK,
                    },
                    Color::White => $color_type::WHITE,
                    Color::Black => $color_type::BLACK,
                    Color::Red => $color_type::RED,
                    Color::Green => $color_type::GREEN,
                    Color::Yellow => $color_type::YELLOW,
                    Color::Blue => $color_type::BLUE,
                    Color::Magenta => $color_type::MAGENTA,
                    Color::Cyan => $color_type::CYAN,

                    Color::LightRed => LIGHT_RED,
                    Color::LightGreen => LIGHT_GREEN,
                    Color::LightYellow => LIGHT_YELLOW,
                    Color::LightBlue => LIGHT_BLUE,
                    Color::LightMagenta => LIGHT_MAGENTA,
                    Color::LightCyan => LIGHT_CYAN,
                    Color::Gray => GRAY,
                    Color::DarkGray => DARK_GRAY,

                    Color::Rgb(r, g, b) => $color_type::new(
                        (r as f32 * R_RATIO) as u8,
                        (g as f32 * G_RATIO) as u8,
                        (b as f32 * B_RATIO) as u8,
                    ),
                    Color::Indexed(_) => todo!("Color::Indexed not implemented yet!"),
                }
            }
        }

        impl<D, const N: usize> Backend for EmbeddedBackend<'_, D, $color_type, N>
        where
            D: DrawTarget<Color = $color_type>,
        {
            fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
            where
                I: Iterator<Item = (u16, u16, &'a Cell)>,
            {
                for (x, y, cell) in content {
                    let position = geometry::Point::new(
                        x as i32 * self.font_regular.character_size.width as i32,
                        y as i32 * self.font_regular.character_size.height as i32,
                    );
                    let mut style_builder = MonoTextStyleBuilder::new()
                        .font(if cell.style().add_modifier.contains(Modifier::BOLD) {
                            &self.font_bold
                        } else {
                            &self.font_regular
                        })
                        .text_color(TermColor(cell.fg, TermColorType::Foreground).into())
                        .background_color(TermColor(cell.bg, TermColorType::Background).into());

                    if cell.style().add_modifier.contains(Modifier::UNDERLINED) {
                        style_builder = style_builder.underline();
                    }
                    if cell.style().add_modifier.contains(Modifier::CROSSED_OUT) {
                        style_builder = style_builder.strikethrough();
                    }
                    let style = style_builder.build();

                    Text::new(cell.symbol(), position + self.char_offset, style)
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

            fn get_cursor_position(&mut self) -> io::Result<Position> {
                // TODO
                Ok(Position::new(0, 0))
            }

            fn set_cursor_position<P: Into<Position>>(
                &mut self,
                #[allow(unused_variables)] position: P,
            ) -> io::Result<()> {
                // TODO
                Ok(())
            }

            fn clear(&mut self) -> io::Result<()> {
                self.display
                    .clear($color_type::BLACK)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, DrawError))
            }

            fn size(&self) -> io::Result<Size> {
                Ok(self.columns_rows)
            }

            fn window_size(&mut self) -> io::Result<WindowSize> {
                Ok(WindowSize {
                    columns_rows: self.columns_rows,
                    pixels: self.pixels,
                })
            }

            fn flush(&mut self) -> io::Result<()> {
                // TODO map error...
                self.display
                    .fill_contiguous(&self.display_area, self.buffer.data)
                    .ok();

                #[cfg(feature = "simulator")]
                self.update_simulation()?;
                Ok(())
            }
        }
    };
}

impl_for_color!(Rgb555);
impl_for_color!(Bgr555);
impl_for_color!(Rgb565);
impl_for_color!(Bgr565);
impl_for_color!(Rgb666);
impl_for_color!(Bgr666);
impl_for_color!(Rgb888);
impl_for_color!(Bgr888);

#[cfg(test)]
mod tests {
    // TODO: tests
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
