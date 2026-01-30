#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

extern crate alloc;

mod button;
mod chart;
mod gauge;
mod helpers;
mod lorem;
mod ratatui_logo;
mod tabs;
mod voltage;

use alloc::boxed::Box;
use button::Button;
use chart::ChartApp;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::main;
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::time::{Duration, Rate};
use gauge::GaugeApp;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::options::{ColorInversion, Orientation, Rotation};
use mousefood::prelude::*;
use ratatui::Terminal;
use ratatui_logo::RatatuiLogoApp;
use tabs::TabsApp;
use voltage::VoltageApp;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    loop {
        log::error!("{panic_info:?}");
    }
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const DISPLAY_OFFSET: (u16, u16) = (52, 40);
const DISPLAY_SIZE: (u16, u16) = (135, 240);

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::info!("Starting Mousefood");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 128 * 1024);

    // Turn on display backlight
    let _backlight = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());

    // Configure SPI
    log::info!("Initializing SPI display");
    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(80))
            .with_mode(Mode::_3),
    )
    .unwrap()
    .with_sck(peripherals.GPIO18)
    .with_mosi(peripherals.GPIO19);

    let cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();

    let dc = Output::new(peripherals.GPIO16, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO23, Level::High, OutputConfig::default());
    let buffer = Box::leak(Box::new([0_u8; 4096]));
    let spi_interface = SpiInterface::new(spi_device, dc, buffer);

    // Configure display
    let mut delay = Delay::new();
    let mut display = Builder::new(ST7789, spi_interface)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .display_offset(DISPLAY_OFFSET.0, DISPLAY_OFFSET.1)
        .display_size(DISPLAY_SIZE.0, DISPLAY_SIZE.1)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut delay)
        .expect("Failed to init display");

    // Setup button
    let button_pin = Input::new(
        peripherals.GPIO0,
        InputConfig::default().with_pull(Pull::Up),
    );
    let mut button = Button::new(button_pin, Duration::from_millis(150));

    // Setup battery voltage reader
    let mut adc_config = AdcConfig::new();
    let mut battery_adc = adc_config.enable_pin(peripherals.GPIO34, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    log::info!("Setup complete, creating the terminal");

    // Setup Mousefood and Ratatui
    let backend = EmbeddedBackend::new(&mut display, Default::default());

    let mut terminal = Terminal::new(backend).unwrap();

    log::info!("Starting render loop");

    loop {
        RatatuiLogoApp::new().run(&mut terminal, &mut button, &delay);
        delay.delay_millis(200);

        TabsApp::new().run(&mut terminal, &mut button, &delay);
        delay.delay_millis(200);

        ChartApp::new().run(&mut terminal, &mut button, &delay);
        delay.delay_millis(200);

        GaugeApp::new().run(&mut terminal, &mut button, &delay);
        delay.delay_millis(200);

        let mut read_voltage = || adc.read_oneshot(&mut battery_adc).ok();
        VoltageApp::new().run(&mut terminal, &mut button, &delay, &mut read_voltage);
        delay.delay_millis(200);
    }
}
