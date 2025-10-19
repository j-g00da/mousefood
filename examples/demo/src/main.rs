mod chart;
mod gauge;
mod helpers;
mod lorem;
mod ratatui_logo;
mod tabs;
mod voltage;

use crate::chart::ChartApp;
use crate::gauge::GaugeApp;
use crate::ratatui_logo::RatatuiLogoApp;
use crate::tabs::TabsApp;
use crate::voltage::VoltageApp;
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::{AnyIOPin, InterruptType, PinDriver};
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::spi::config::MODE_3;
use esp_idf_svc::hal::spi::{SpiConfig, SpiDeviceDriver, SpiDriverConfig};
use esp_idf_svc::hal::task::notification::Notification;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::options::{ColorInversion, Orientation, Rotation};
use mousefood::prelude::*;
use std::num::NonZeroU32;
use std::thread;
use std::time::Duration;

const DISPLAY_OFFSET: (u16, u16) = (52, 40);
const DISPLAY_SIZE: (u16, u16) = (135, 240);

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // Turn on display backlight
    let mut backlight = PinDriver::output(peripherals.pins.gpio4).unwrap();
    backlight.set_high().unwrap();

    // Configure SPI
    let config = SpiConfig::new()
        .write_only(true)
        .baudrate(80u32.MHz().into())
        .data_mode(MODE_3);
    let spi_device = SpiDeviceDriver::new_single(
        peripherals.spi2,
        peripherals.pins.gpio18,
        peripherals.pins.gpio19,
        Option::<AnyIOPin>::None,
        Some(peripherals.pins.gpio5),
        &SpiDriverConfig::new(),
        &config,
    )
    .unwrap();
    let buffer = Box::leak(Box::new([0_u8; 4096]));
    let spi_interface = SpiInterface::new(
        spi_device,
        PinDriver::output(peripherals.pins.gpio16).unwrap(),
        buffer,
    );

    // Configure display
    let mut delay = Ets;
    let mut display = Builder::new(ST7789, spi_interface)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(PinDriver::output(peripherals.pins.gpio23).unwrap())
        .display_offset(DISPLAY_OFFSET.0, DISPLAY_OFFSET.1)
        .display_size(DISPLAY_SIZE.0, DISPLAY_SIZE.1)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut delay)
        .expect("Failed to init display");

    // Setup button interrupt
    let mut button = PinDriver::input(peripherals.pins.gpio0).unwrap();
    button.set_interrupt_type(InterruptType::NegEdge).unwrap();
    let mut notification = Notification::new();
    let notifier = notification.notifier();
    unsafe {
        button
            .subscribe(move || {
                notifier.notify_and_yield(NonZeroU32::new(1).unwrap());
            })
            .unwrap();
    }

    // Setup battery voltage reader
    let adc_driver = AdcDriver::new(peripherals.adc1).unwrap();
    let mut battery_adc_channel = AdcChannelDriver::new(
        &adc_driver,
        peripherals.pins.gpio34,
        &AdcChannelConfig {
            attenuation: DB_11,
            calibration: Calibration::Line,
            resolution: Resolution::Resolution12Bit,
        },
    )
    .unwrap();

    // Setup Mousefood and Ratatui
    let backend = EmbeddedBackend::new(&mut display, Default::default());
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        RatatuiLogoApp::new()
            .run(&mut terminal, &mut notification, &mut button)
            .unwrap();

        thread::sleep(Duration::from_millis(200));

        TabsApp::new()
            .run(&mut terminal, &mut notification, &mut button)
            .unwrap();

        thread::sleep(Duration::from_millis(200));

        ChartApp::new()
            .run(&mut terminal, &mut notification, &mut button)
            .unwrap();

        thread::sleep(Duration::from_millis(200));

        GaugeApp::new()
            .run(&mut terminal, &mut notification, &mut button)
            .unwrap();

        thread::sleep(Duration::from_millis(200));

        VoltageApp::new()
            .run(
                &mut terminal,
                &mut notification,
                &mut button,
                &adc_driver,
                &mut battery_adc_channel,
            )
            .unwrap();

        thread::sleep(Duration::from_millis(200));
    }
}
