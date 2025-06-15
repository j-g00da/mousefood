use crate::macros::for_all_rgb_colors;
use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, BinaryColor, Rgb555, Rgb565, Rgb666, Rgb888, RgbColor,
};
use ratatui::style::Color;

pub enum TermColorType {
    Foreground,
    Background,
}

pub struct TermColor(pub Color, pub TermColorType);

macro_rules! impl_from_term_color {
    (
        $color_type:ident
    ) => {
        impl From<TermColor> for $color_type {
            fn from(color: TermColor) -> Self {
                const LIGHT_RED: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G / 2, Rgb888::MAX_B / 2);
                const LIGHT_GREEN: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G, Rgb888::MAX_B / 2);
                const LIGHT_YELLOW: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G, Rgb888::MAX_B / 2);
                const LIGHT_BLUE: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G / 2, Rgb888::MAX_B);
                const LIGHT_MAGENTA: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G / 2, Rgb888::MAX_B);
                const LIGHT_CYAN: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G, Rgb888::MAX_B);
                const GRAY: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G / 2, Rgb888::MAX_B / 2);
                const DARK_GRAY: Rgb888 = Rgb888::new(
                    (2.0 / 3.0 * (Rgb888::MAX_R as f32)) as u8,
                    (2.0 / 3.0 * (Rgb888::MAX_G as f32)) as u8,
                    (2.0 / 3.0 * (Rgb888::MAX_B as f32)) as u8,
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

                    Color::LightRed => LIGHT_RED.into(),
                    Color::LightGreen => LIGHT_GREEN.into(),
                    Color::LightYellow => LIGHT_YELLOW.into(),
                    Color::LightBlue => LIGHT_BLUE.into(),
                    Color::LightMagenta => LIGHT_MAGENTA.into(),
                    Color::LightCyan => LIGHT_CYAN.into(),
                    Color::Gray => GRAY.into(),
                    Color::DarkGray => DARK_GRAY.into(),

                    Color::Rgb(r, g, b) => Rgb888::new(r, g, b).into(),
                    Color::Indexed(_) => todo!("Color::Indexed not implemented yet!"),
                }
            }
        }
    };
}

for_all_rgb_colors!(impl_from_term_color);

impl From<TermColor> for BinaryColor {
    fn from(color: TermColor) -> Self {
        match color.0 {
            Color::Black => BinaryColor::Off,
            Color::White => BinaryColor::On,
            // Fallback
            _ => match color.1 {
                TermColorType::Foreground => BinaryColor::Off,
                TermColorType::Background => BinaryColor::On,
            },
        }
    }
}

#[cfg(feature = "epd-weact")]
impl From<TermColor> for weact_studio_epd::Color {
    fn from(color: TermColor) -> Self {
        BinaryColor::from(color).into()
    }
}

#[cfg(feature = "epd-weact")]
impl From<TermColor> for weact_studio_epd::TriColor {
    fn from(color: TermColor) -> Self {
        match color.0 {
            Color::White => weact_studio_epd::TriColor::White,
            Color::Black => weact_studio_epd::TriColor::Black,
            Color::Red => weact_studio_epd::TriColor::Red,
            // Fallback
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
                    let output: $color_type = TermColor(color_from, color_type).into();
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
    fn into_binary_color(
        #[case] color_type: TermColorType,
        #[case] color_from: Color,
        #[case] color_into: BinaryColor,
    ) {
        let output: BinaryColor = TermColor(color_from, color_type).into();
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
        let output: weact_studio_epd::Color = TermColor(color_from, color_type).into();
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
        let output: weact_studio_epd::TriColor = TermColor(color_from, color_type).into();
        assert_eq!(output, color_into);
    }
}
