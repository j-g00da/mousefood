use crate::colors::{TermColor, TermColorType};
use alloc::vec::IntoIter;
use embedded_graphics::Pixel;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::primitives::Rectangle;
use ratatui::style::Color;

pub(crate) struct HeapBuffer<C: PixelColor + Copy> {
    pub data: Vec<C>,
    bounding_box: Rectangle,
}

impl<C: PixelColor + From<TermColor>> HeapBuffer<C> {
    pub fn new(bounding_box: Rectangle) -> HeapBuffer<C> {
        Self {
            data: vec![
                TermColor(Color::Reset, TermColorType::Background).into();
                (bounding_box.size.width * bounding_box.size.height) as usize
            ],
            bounding_box,
        }
    }
}

impl<C: PixelColor> IntoIterator for HeapBuffer<C> {
    type Item = C;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<C: PixelColor> IntoIterator for &HeapBuffer<C> {
    type Item = C;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.clone().into_iter()
    }
}

impl<C: PixelColor> Dimensions for HeapBuffer<C> {
    fn bounding_box(&self) -> Rectangle {
        self.bounding_box
    }
}

impl<C: PixelColor> DrawTarget for HeapBuffer<C> {
    type Color = C;
    type Error = std::io::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let idx_end = self.data.len() - 1;
        for Pixel(point, color) in pixels {
            let idx = point.y as usize * self.bounding_box.size.width as usize + point.x as usize;
            self.data[idx.clamp(0, idx_end)] = color;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::mono_font::MonoTextStyle;
    use embedded_graphics::mono_font::ascii::FONT_4X6;
    use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
    use embedded_graphics::prelude::*;
    use embedded_graphics::text::Text;

    #[fixture]
    fn display() -> MockDisplay<Rgb888> {
        MockDisplay::new()
    }

    #[fixture]
    fn heap_buffer() -> HeapBuffer<Rgb888> {
        HeapBuffer::new(Rectangle::new(Point::zero(), Size::new(16, 8)))
    }

    #[fixture]
    fn test_text<'a>() -> (Text<'a, MonoTextStyle<'a, Rgb888>>, &'a [&'a str]) {
        (
            Text::new(
                "Test",
                Point::new(0, 6),
                MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE),
            ),
            &[
                "KKKKKKKKKKKKKKKK",
                "KKKKKKKKKKKKKKKK",
                "WWWKKKKKKKKKKWKK",
                "KWKKKWKKKWWKWWWK",
                "KWKKWKWKWWKKKWKK",
                "KWKKWWKKKKWKKWKK",
                "KWKKKWWKWWKKKKWK",
                "KKKKKKKKKKKKKKKK",
            ],
        )
    }

    #[rstest]
    fn test_heap_buffer(
        mut display: MockDisplay<Rgb888>,
        mut heap_buffer: HeapBuffer<Rgb888>,
        #[from(test_text)] (text, expected): (Text<MonoTextStyle<Rgb888>>, &[&str]),
    ) {
        text.draw(&mut heap_buffer).unwrap();

        display
            .fill_contiguous(&heap_buffer.bounding_box(), heap_buffer)
            .unwrap();

        display.assert_pattern(expected);
    }

    #[rstest]
    fn test_heap_buffer_as_ref(
        mut display: MockDisplay<Rgb888>,
        mut heap_buffer: HeapBuffer<Rgb888>,
        #[from(test_text)] (text, expected): (Text<MonoTextStyle<Rgb888>>, &[&str]),
    ) {
        text.draw(&mut heap_buffer).unwrap();

        display
            .fill_contiguous(&heap_buffer.bounding_box(), &heap_buffer)
            .unwrap();

        display.assert_pattern(expected);
    }
}
