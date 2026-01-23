#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{clock::CpuClock, delay::Delay, main, time::Rate};

use alloc::boxed::Box;
use embedded_hal_bus::spi::ExclusiveDevice;
use epd_waveshare::{epd2in9_v2::*, prelude::*};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::{
    Mode,
    master::{Config, Spi},
};
use mousefood::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 128 * 1024);

    // setup spi
    let sck = peripherals.GPIO12;
    let mosi = peripherals.GPIO11;
    let mut delay = Delay::default();

    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(30))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sck)
    .with_mosi(mosi);

    let busy = Input::new(
        peripherals.GPIO18,
        InputConfig::default().with_pull(Pull::None),
    );
    let rst = Output::new(peripherals.GPIO17, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO16, Level::High, OutputConfig::default());
    let cs = Output::new(peripherals.GPIO10, Level::High, OutputConfig::default());
    let mut spi_device = ExclusiveDevice::new(spi, cs, delay).expect("could not init spi device");

    // setup display
    let mut epd = Epd2in9::new(&mut spi_device, busy, dc, rst, &mut delay, None).unwrap();
    let mut display = Display2in9::default();
    display.set_rotation(DisplayRotation::Rotate90);

    // setup mousefood
    let backend = EmbeddedBackendConfig {
        flush_callback: Box::new(move |display: &mut Display2in9| {
            epd.update_and_display_frame(&mut spi_device, display.buffer(), &mut delay)
                .unwrap();
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(&mut display, backend);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(draw).unwrap();

    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text.white()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered().title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
