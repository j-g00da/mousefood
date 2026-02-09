# ![Mousefood](https://github.com/ratatui/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/logo/mousefood.svg?raw=true)

[![Crate](https://img.shields.io/crates/v/mousefood?logo=rust&style=flat-square&color=ebe94f)](https://crates.io/crates/mousefood)
[![Docs](https://img.shields.io/docsrs/mousefood?logo=rust&style=flat-square)](https://docs.rs/mousefood)
[![CI](https://img.shields.io/github/actions/workflow/status/ratatui/mousefood/ci.yml?style=flat-square&logo=github)](https://github.com/ratatui/mousefood/blob/main/.github/workflows/ci.yml)
[![Deps](https://deps.rs/crate/mousefood/latest/status.svg?style=flat-square)](https://deps.rs/crate/mousefood)

**Mousefood** - a no-std
[embedded-graphics](https://crates.io/crates/embedded-graphics) backend
for [Ratatui](https://crates.io/crates/ratatui)!

<div align="center">

![demo](https://github.com/ratatui/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/demo.jpg?raw=true)
![animated demo](https://github.com/ratatui/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/demo.gif?raw=true)

</div>

## Quickstart

Add mousefood as a dependency:

```shell
cargo add mousefood
```

Exemplary setup:

```rust
use mousefood::embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};
use mousefood::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{Frame, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // replace this with your display driver
    // e.g. ILI9341, ST7735, SSD1306, etc.
    let mut display = MockDisplay::<Rgb888>::new();

    let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(draw)?;
    Ok(())
}

fn draw(frame: &mut Frame) {
    let block = Block::bordered().title("Mousefood");
    let paragraph = Paragraph::new("Hello from Mousefood!").block(block);
    frame.render_widget(paragraph, frame.area());
}
```

### Special characters

Embedded-graphics includes bitmap fonts that have a very limited
set of characters to save space (ASCII, ISO 8859 or JIS X0201).
This makes it impossible to draw most of Ratatui's widgets,
which heavily use box-drawing glyphs, Braille,
and other special characters.

Mousefood by default uses [`embedded-graphics-unicodefonts`](https://crates.io/crates/embedded-graphics-unicodefonts),
which provides embedded-graphics fonts with a much larger set of characters.

#### Alternatives

In order to save space and [speed up rendering](#performance-and-hardware-support),
the `fonts` feature can be disabled by turning off the default crate features.
[`ibm437`](https://crates.io/crates/ibm437) is a good alternative that includes
some drawing characters, but is not as large as embedded-graphics-unicodefonts.

### Bold and italic fonts

Bold and italic modifiers are supported, but this requires providing fonts
through `EmbeddedBackendConfig`.
If only regular font is provided, it serves as a fallback.
All fonts must be of the same size.

```rust
use mousefood::embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig, fonts};
use ratatui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut display = MockDisplay::<Rgb888>::new();
    let config = EmbeddedBackendConfig {
        font_regular: fonts::MONO_6X13,
        font_bold: Some(fonts::MONO_6X13_BOLD),
        font_italic: Some(fonts::MONO_6X13_ITALIC),
        ..Default::default()
    };
    let backend = EmbeddedBackend::new(&mut display, config);
    let _terminal = Terminal::new(backend)?;
    Ok(())
}
```

<div align="center">
<img alt="Bold and Italic fonts"
     src="https://github.com/ratatui/mousefood/blob/6640da9402794ea8f9370e0dc2b4bd1ebf2c6356/assets/bold_italic.png?raw=true"
     style="max-width: 640px"/>
</div>

### Color theme

Colors can be remapped using `color_theme` on `EmbeddedBackendConfig`.
By default the ANSI palette is used.

```rust
use mousefood::{ColorTheme, EmbeddedBackend, EmbeddedBackendConfig};
use mousefood::embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut display = MockDisplay::<Rgb888>::new();
    let theme = ColorTheme {
        background: Rgb888::new(5, 5, 5),
        foreground: Rgb888::new(240, 240, 240),
        yellow: Rgb888::new(255, 200, 0),
        ..ColorTheme::ansi()
    };

    let config = EmbeddedBackendConfig {
        color_theme: theme,
        ..Default::default()
    };
    let backend = EmbeddedBackend::new(&mut display, config);
    Ok(())
}
```

#### Built-in themes

Mousefood includes popular color themes that can be used directly:

- `ColorTheme::ansi()` - Standard ANSI colors (default)
- `ColorTheme::tokyo_night()` - Tokyo Night dark theme with blue/purple tones

### Simulator

Mousefood can be run in a simulator using
[embedded-graphics-simulator](https://crates.io/crates/embedded-graphics-simulator) crate.

![Screenshot of a window running the simulator with a mousefood application](https://github.com/ratatui/mousefood/blob/66d4010deed18f755cc3148a7f682f4119b7f664/assets/simulator.png?raw=true)

Run simulator example:

```shell
git clone https://github.com/ratatui/mousefood.git
cd mousefood/examples/simulator
cargo run
```

For more details, view the [simulator example](examples/simulator).

### EPD support

#### WeAct Studio

<div align="center">

![WeAct epd demo](https://github.com/ratatui/mousefood/blob/fa70cdd46567a51895caf10c44fff4104602e880/assets/epd-weact.jpg?raw=true)

</div>

Support for EPD (e-ink displays) produced by WeAct Studio
(`weact-studio-epd` driver) can be enabled using `epd-weact` feature.

This driver requires some additional configuration.
Follow the [`weact-studio-epd`](https://docs.rs/weact-studio-epd)
crate docs and apply the same `flush_callback` pattern used in the [Waveshare example below](#waveshare).

<details>
  <summary>Setup example</summary>

EPD drivers include their own internal buffers, so the mousefood framebuffer
adds memory overhead with no benefit. Disable default features to turn it off:

```toml
[dependencies]
mousefood = { version = "*", default-features = false, features = ["epd-weact"] }
```

```rust,ignore
use mousefood::prelude::*;
use weact_studio_epd::graphics::Display290BlackWhite;
use weact_studio_epd::WeActStudio290BlackWhiteDriver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure SPI + GPIO + delay provider for your board.
    // let (spi_interface, busy, rst, delay) = ...;

    let mut driver = WeActStudio290BlackWhiteDriver::new(spi_interface, busy, rst, delay);
    let mut display = Display290BlackWhite::new();

    driver.init()?;

    let config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |d| {
            driver.full_update(d).expect("epd update failed");
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(&mut display, config);
    let _terminal = Terminal::new(backend)?;
    Ok(())
}
```

</details>

#### Waveshare

Support for EPD (e-ink displays) produced by Waveshare Electronics
(`epd-waveshare` driver) can be enabled using `epd-waveshare` feature.

<details>
  <summary>Setup example</summary>

EPD drivers include their own internal buffers, so the mousefood framebuffer
adds memory overhead with no benefit. Disable default features to turn it off:

```toml
[dependencies]
mousefood = { version = "*", default-features = false, features = ["epd-waveshare"] }
```

```rust,ignore
use mousefood::prelude::*;
use epd_waveshare::{epd2in9_v2::*, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure SPI + GPIO + delay provider for your board.
    // let (mut spi_device, busy, dc, rst, mut delay) = ...;

    let mut epd = Epd2in9::new(&mut spi_device, busy, dc, rst, &mut delay, None)?;
    let mut display = Display2in9::default();

    let config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |d| {
            epd.update_and_display_frame(&mut spi_device, d.buffer(), &mut delay)
                .expect("epd update failed");
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(&mut display, config);
    let _terminal = Terminal::new(backend)?;
    Ok(())
}
```

</details>

See the full embedded example at [`examples/epd-waveshare-demo`](https://github.com/ratatui/mousefood/tree/main/examples/epd-waveshare-demo).

## Performance and hardware support

Flash memory on most embedded devices is very limited. Additionally,
to achieve high frame rate when using the `fonts` feature,
it is recommended to use `opt-level = 3`,
which can make the resulting binary even larger.

Mousefood is hardware-agnostic.
Successfully tested on:

### Microcontrollers

- ESP32 (Xtensa)
- ESP32-C6 (RISC-V)
- STM32
- RP2040
- RP2350

### Display drivers

For every driver, the list of displays is not exhaustive.

- [ssd1306](https://crates.io/crates/ssd1306) for SSD1306
- [mipidsi](https://crates.io/crates/mipidsi) for ILI9341, ST7735, etc.
- [epd-waveshare](https://crates.io/crates/epd-waveshare) for e-paper displays from Waveshare
  (requires enabling `epd-waveshare` feature)
- [weact-studio-epd](https://crates.io/crates/weact-studio-epd) for e-paper displays
  from WeAct Studio (requires enabling `epd-weact` feature)

Send a pull request to add your microcontroller or display driver here!

## Docs

Full API docs are available on [docs.rs](https://docs.rs/mousefood).

## Contributing

All contributions are welcome!

Before opening a pull request, please read the [contributing guidelines](./CONTRIBUTING.md).

## Built with Mousefood

Here are some projects built using Mousefood:

- [Tuitar](https://github.com/orhun/tuitar) - A portable guitar training tool.
- [Mnyaoo32](https://github.com/intuis/mnyaoo32) - An eccentric way to consume IRC messages using ESP32.
- [Phone-OS](https://github.com/Julien-cpsn/Phone-OS) - A modern phone OS for ESP32 CYD.

Send a pull request to add your project here!

## License

[![License MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square&color=8d97b3)](LICENSE-MIT)
[![License Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square&color=8d97b3)](LICENSE-APACHE)

Mousefood is dual-licensed under
[Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) terms.
