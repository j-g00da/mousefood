# ![Mousefood](https://github.com/j-g00da/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/logo/mousefood.svg?raw=true)

[![Crate](https://img.shields.io/crates/v/mousefood?logo=rust&style=flat-square&color=ebe94f)](https://crates.io/crates/mousefood)
[![Docs](https://img.shields.io/docsrs/mousefood?logo=rust&style=flat-square)](https://docs.rs/mousefood)
[![CI](https://img.shields.io/github/actions/workflow/status/j-g00da/mousefood/ci.yml?style=flat-square&logo=github)](https://github.com/j-g00da/mousefood/blob/main/.github/workflows/ci.yml)
[![Deps](https://deps.rs/crate/mousefood/latest/status.svg?style=flat-square)](https://deps.rs/crate/mousefood)

**Mousefood** - a no-std
[embedded-graphics](https://crates.io/crates/embedded-graphics) backend
for [Ratatui](https://crates.io/crates/ratatui)!

<div align="center">

![demo](https://github.com/j-g00da/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/demo.jpg?raw=true)
![animated demo](https://github.com/j-g00da/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/demo.gif?raw=true)

</div>

## Quickstart

Add mousefood as a dependency:

```shell
cargo add mousefood
```

Exemplary setup:

```rust
use mousefood::prelude::*;

fn main() -> Result<(), std::io::Error> {
    // Any embedded_graphics DrawTarget
    let mut display = MyDrawTarget::new();
    
    let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(...)?;
    }
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
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig, fonts};

let config = EmbeddedBackendConfig {
    font_regular: fonts::MONO_6X13,
    font_bold: Some(fonts::MONO_6X13_BOLD),
    font_italic: Some(fonts::MONO_6X13_ITALIC),
    ..Default::default()
};
let backend = EmbeddedBackend::new(&mut display, config);
```

<div align="center">
<img alt="Bold and Italic fonts"
     src="https://github.com/j-g00da/mousefood/blob/6640da9402794ea8f9370e0dc2b4bd1ebf2c6356/assets/bold_italic.png?raw=true"
     style="max-width: 640px"/>
</div>

### Simulator

Mousefood can be run in a simulator using
[embedded-graphics-simulator](https://crates.io/crates/embedded-graphics-simulator) crate.

![Screenshot of a window running the simulator with a mousefood application](https://github.com/j-g00da/mousefood/blob/66d4010deed18f755cc3148a7f682f4119b7f664/assets/simulator.png?raw=true)

Run simulator example:

```shell
git clone https://github.com/j-g00da/mousefood.git
cd mousefood/examples/simulator
cargo run
```

For more details, view the [simulator example](examples/simulator).

### EPD support

<div align="center">

![WeAct epd demo](https://github.com/j-g00da/mousefood/blob/fa70cdd46567a51895caf10c44fff4104602e880/assets/epd-weact.jpg?raw=true)

</div>

Support for EPD (e-ink displays) produced by WeAct Studio
(`weact-studio-epd` driver) can be enabled using `epd-weact` feature.
This driver requires some additional configuration:

```rust
use mousefood::prelude::*;
use weact_studio_epd::graphics::Display290BlackWhite;
use weact_studio_epd::WeActStudio290BlackWhiteDriver;

// Configure SPI
// (...)

let mut driver = WeActStudio290BlackWhiteDriver::new(spi_interface, busy, rst, delay);
let mut display = Display290BlackWhite::new();

driver.init().unwrap();

let config = EmbeddedBackendConfig {
    flush_callback: Box::new(move |d| { driver.full_update(d).unwrap(); }),
    ..Default::default()
};
let backend = EmbeddedBackend::new(&mut display, config);
```

Support for `epd_waveshare` driver is planned in the future.

## Performance and hardware support

Flash memory on most embedded devices is very limited. Additionally,
to achieve high frame rate when using the `fonts` feature,
it is recommended to use `opt-level = 3`,
which can make the resulting binary even larger.

Mousefood is hardware-agnostic.
Successfully tested on:

- esp32 (base model, 4MB flash)
- esp32c6 (16MB flash)

## Docs

Full API docs are available on [docs.rs](https://docs.rs/mousefood).

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
