use crate::macros::for_all_rgb_colors;
use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, BinaryColor, Rgb555, Rgb565, Rgb666, Rgb888, RgbColor,
};
use ratatui_core::style::Color;

/// Defines how ratatui colors should be mapped to the display colors.
#[derive(Clone, Copy)]
pub struct ColorTheme {
    /// Default foreground color when `Color::Reset` is used.
    pub foreground: Rgb888,
    /// Default background color when `Color::Reset` is used.
    pub background: Rgb888,
    /// ANSI white mapping.
    pub white: Rgb888,
    /// ANSI black mapping.
    pub black: Rgb888,
    /// ANSI red mapping.
    pub red: Rgb888,
    /// ANSI green mapping.
    pub green: Rgb888,
    /// ANSI yellow mapping.
    pub yellow: Rgb888,
    /// ANSI blue mapping.
    pub blue: Rgb888,
    /// ANSI magenta mapping.
    pub magenta: Rgb888,
    /// ANSI cyan mapping.
    pub cyan: Rgb888,
    /// ANSI bright red mapping.
    pub light_red: Rgb888,
    /// ANSI bright green mapping.
    pub light_green: Rgb888,
    /// ANSI bright yellow mapping.
    pub light_yellow: Rgb888,
    /// ANSI bright blue mapping.
    pub light_blue: Rgb888,
    /// ANSI bright magenta mapping.
    pub light_magenta: Rgb888,
    /// ANSI bright cyan mapping.
    pub light_cyan: Rgb888,
    /// ANSI gray mapping.
    pub gray: Rgb888,
    /// ANSI dark gray mapping.
    pub dark_gray: Rgb888,
}

impl ColorTheme {
    /// ANSI color palette used by default.
    pub const fn ansi() -> Self {
        Self {
            foreground: Rgb888::WHITE,
            background: Rgb888::BLACK,
            white: Rgb888::WHITE,
            black: Rgb888::BLACK,
            red: Rgb888::RED,
            green: Rgb888::GREEN,
            yellow: Rgb888::YELLOW,
            blue: Rgb888::BLUE,
            magenta: Rgb888::MAGENTA,
            cyan: Rgb888::CYAN,
            light_red: Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G / 2, Rgb888::MAX_B / 2),
            light_green: Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G, Rgb888::MAX_B / 2),
            light_yellow: Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G, Rgb888::MAX_B / 2),
            light_blue: Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G / 2, Rgb888::MAX_B),
            light_magenta: Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G / 2, Rgb888::MAX_B),
            light_cyan: Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G, Rgb888::MAX_B),
            gray: Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G / 2, Rgb888::MAX_B / 2),
            dark_gray: Rgb888::new(170, 170, 170),
        }
    }

    pub(crate) fn resolve(&self, color: Color, color_type: TermColorType) -> Rgb888 {
        match color {
            Color::Reset => match color_type {
                TermColorType::Foreground => self.foreground,
                TermColorType::Background => self.background,
            },
            Color::White => self.white,
            Color::Black => self.black,
            Color::Red => self.red,
            Color::Green => self.green,
            Color::Yellow => self.yellow,
            Color::Blue => self.blue,
            Color::Magenta => self.magenta,
            Color::Cyan => self.cyan,

            Color::LightRed => self.light_red,
            Color::LightGreen => self.light_green,
            Color::LightYellow => self.light_yellow,
            Color::LightBlue => self.light_blue,
            Color::LightMagenta => self.light_magenta,
            Color::LightCyan => self.light_cyan,
            Color::Gray => self.gray,
            Color::DarkGray => self.dark_gray,

            Color::Rgb(r, g, b) => Rgb888::new(r, g, b),
            Color::Indexed(_) => Rgb888::BLACK,
        }
    }
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self::ansi()
    }
}

#[derive(Clone, Copy)]
pub enum TermColorType {
    Foreground,
    Background,
}

#[derive(Clone, Copy)]
pub struct TermColor<'a>(pub Color, pub TermColorType, pub &'a ColorTheme);

impl<'a> TermColor<'a> {
    pub fn new(color: Color, color_type: TermColorType, theme: &'a ColorTheme) -> Self {
        Self(color, color_type, theme)
    }

    fn to_rgb888(&self) -> Rgb888 {
        self.2.resolve(self.0, self.1)
    }
}

macro_rules! impl_from_term_color {
    (
        $color_type:ident
    ) => {
        impl<'a> From<TermColor<'a>> for $color_type {
            fn from(color: TermColor<'a>) -> Self {
                color.to_rgb888().into()
            }
        }
    };
}

for_all_rgb_colors!(impl_from_term_color);

impl<'a> From<TermColor<'a>> for BinaryColor {
    fn from(color: TermColor<'a>) -> Self {
        match color.to_rgb888() {
            rgb if rgb == Rgb888::BLACK => BinaryColor::Off,
            rgb if rgb == Rgb888::WHITE => BinaryColor::On,
            _ => match color.1 {
                TermColorType::Foreground => BinaryColor::On,
                TermColorType::Background => BinaryColor::Off,
            },
        }
    }
}

#[cfg(feature = "epd-weact")]
impl<'a> From<TermColor<'a>> for weact_studio_epd::Color {
    fn from(color: TermColor<'a>) -> Self {
        BinaryColor::from(color).into()
    }
}

#[cfg(feature = "epd-weact")]
impl<'a> From<TermColor<'a>> for weact_studio_epd::TriColor {
    fn from(color: TermColor<'a>) -> Self {
        let rgb = color.to_rgb888();
        match rgb {
            rgb if rgb == Rgb888::WHITE => weact_studio_epd::TriColor::White,
            rgb if rgb == Rgb888::BLACK => weact_studio_epd::TriColor::Black,
            rgb if rgb == Rgb888::RED => weact_studio_epd::TriColor::Red,
            _ => match color.1 {
                TermColorType::Foreground => weact_studio_epd::TriColor::Black,
                TermColorType::Background => weact_studio_epd::TriColor::White,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Color::*;
    use TermColorType::*;
    use paste::paste;
    use rstest::rstest;

    const TEST_THEME: ColorTheme = ColorTheme::ansi();

    fn themed(color_type: TermColorType, color_from: Color) -> TermColor<'static> {
        TermColor::new(color_from, color_type, &TEST_THEME)
    }

    macro_rules! into_eg_color {
        ($color_type:ident) => {
            paste! {
                #[rstest]
                #[case(Foreground, Reset, $color_type::WHITE)]
                #[case(Background, Reset, $color_type::BLACK)]
                #[case(Foreground, White, $color_type::WHITE)]
                #[case(Background, White, $color_type::WHITE)]
                #[case(Foreground, Black, $color_type::BLACK)]
                #[case(Background, Black, $color_type::BLACK)]
                #[case(Foreground, Red, $color_type::RED)]
                #[case(Background, Red, $color_type::RED)]
                #[case(Foreground, Yellow, $color_type::YELLOW)]
                #[case(Background, Yellow, $color_type::YELLOW)]
                #[case(Foreground, Magenta, $color_type::MAGENTA)]
                #[case(Background, Magenta, $color_type::MAGENTA)]
                #[case(Foreground, Cyan, $color_type::CYAN)]
                #[case(Background, Cyan, $color_type::CYAN)]
                #[case(Foreground, LightRed, Rgb888::new(255, 127, 127).into())]
                #[case(Background, LightRed, Rgb888::new(255, 127, 127).into())]
                #[case(Foreground, LightGreen, Rgb888::new(127, 255, 127).into())]
                #[case(Background, LightGreen, Rgb888::new(127, 255, 127).into())]
                #[case(Foreground, LightYellow, Rgb888::new(255, 255, 127).into())]
                #[case(Background, LightYellow, Rgb888::new(255, 255, 127).into())]
                #[case(Foreground, LightBlue, Rgb888::new(127, 127, 255).into())]
                #[case(Background, LightBlue, Rgb888::new(127, 127, 255).into())]
                #[case(Foreground, LightMagenta, Rgb888::new(255, 127, 255).into())]
                #[case(Background, LightMagenta, Rgb888::new(255, 127, 255).into())]
                #[case(Foreground, LightCyan, Rgb888::new(127, 255, 255).into())]
                #[case(Background, LightCyan, Rgb888::new(127, 255, 255).into())]
                #[case(Foreground, Gray, Rgb888::new(127, 127, 127).into())]
                #[case(Background, Gray, Rgb888::new(127, 127, 127).into())]
                #[case(Foreground, DarkGray, Rgb888::new(170, 170, 170).into())]
                #[case(Background, DarkGray, Rgb888::new(170, 170, 170).into())]
                #[case(Foreground, Rgb(50, 100, 200), Rgb888::new(50, 100, 200).into())]
                #[case(Background, Rgb(50, 100, 200), Rgb888::new(50, 100, 200).into())]
                #[case(Foreground, Rgb(123, 23, 3), Rgb888::new(123, 23, 3).into())]
                #[case(Background, Rgb(123, 23, 3), Rgb888::new(123, 23, 3).into())]
                fn [<into_ $color_type:lower>] (
                    #[case] color_type: TermColorType,
                    #[case] color_from: Color,
                    #[case] color_into: $color_type
                ) {
                    let output: $color_type = themed(color_type, color_from).into();
                    assert_eq!(output, color_into);
                }
            }
        };
    }
    for_all_rgb_colors!(into_eg_color);

    #[rstest]
    #[case(Foreground, Black, BinaryColor::Off)]
    #[case(Background, Black, BinaryColor::Off)]
    #[case(Foreground, White, BinaryColor::On)]
    #[case(Background, White, BinaryColor::On)]
    #[case(Background, Reset, BinaryColor::Off)]
    #[case(Foreground, Reset, BinaryColor::On)]
    fn into_binary_color(
        #[case] color_type: TermColorType,
        #[case] color_from: Color,
        #[case] color_into: BinaryColor,
    ) {
        let output: BinaryColor = themed(color_type, color_from).into();
        assert_eq!(output, color_into);
    }

    #[cfg(feature = "epd-weact")]
    #[rstest]
    #[case(Foreground, Black, weact_studio_epd::Color::Black)]
    #[case(Background, Black, weact_studio_epd::Color::Black)]
    #[case(Foreground, White, weact_studio_epd::Color::White)]
    #[case(Background, White, weact_studio_epd::Color::White)]
    fn into_weact_color(
        #[case] color_type: TermColorType,
        #[case] color_from: Color,
        #[case] color_into: weact_studio_epd::Color,
    ) {
        let output: weact_studio_epd::Color = themed(color_type, color_from).into();
        assert_eq!(output, color_into);
    }

    #[cfg(feature = "epd-weact")]
    #[rstest]
    #[case(Foreground, Black, weact_studio_epd::TriColor::Black)]
    #[case(Background, Black, weact_studio_epd::TriColor::Black)]
    #[case(Foreground, White, weact_studio_epd::TriColor::White)]
    #[case(Background, White, weact_studio_epd::TriColor::White)]
    #[case(Foreground, Red, weact_studio_epd::TriColor::Red)]
    #[case(Background, Red, weact_studio_epd::TriColor::Red)]
    fn into_weact_tricolor(
        #[case] color_type: TermColorType,
        #[case] color_from: Color,
        #[case] color_into: weact_studio_epd::TriColor,
    ) {
        let output: weact_studio_epd::TriColor = themed(color_type, color_from).into();
        assert_eq!(output, color_into);
    }
}
