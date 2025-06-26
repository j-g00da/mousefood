//! # Simulator
//!
//! Run mousefood apps on your computer inside a simulator! Uses [embedded-graphics-simulator](https://crates.io/crates/embedded-graphics-simulator).
//!
//! ## Requirements
//!
//! This app requires [SDL2](https://wiki.libsdl.org/SDL2/Installation) to be installed.
//!
//! If you use [nix](https://nixos.org) you can run `nix-shell -p SDL2`
//! before running the application.
//!
//! ## Run
//!
//! To start this demo, simply run:
//!
//! ```shell
//! cargo run -p simulator
//! ```
//!
//! A window will open with the simulator running.

use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use mousefood::buffered_display::{BufferedDisplay, DrawTarget, FlushError};
use mousefood::embedded_graphics::geometry;
use mousefood::error::Error;
use mousefood::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal, style::*};

type MousefoodColor = Bgr565;
type MousefoodDisplay = SimulatorDisplay<MousefoodColor>;

struct MousefoodSimulator {
    window: Window,
    display: SimulatorDisplay<MousefoodColor>,
}

impl BufferedDisplay<MousefoodDisplay, MousefoodColor> for MousefoodSimulator {
    // Define where widgets will be drawn
    fn draw_target(&mut self) -> &mut impl DrawTarget<Color = MousefoodColor> {
        &mut self.display
    }

    // Define how to display drawn images
    fn flush(&mut self) -> Result<(), FlushError> {
        // Show next image on the window
        self.window.update(&self.display);

        // Stop showing images if window gets closed
        if self.window.events().any(|e| e == SimulatorEvent::Quit) {
            return Err(FlushError("simulator window closed".into()));
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let mut display = MousefoodSimulator {
        // Create simulator window
        window: Window::new(
            "mousefood simulator",
            &OutputSettings {
                scale: 4,
                max_fps: 30,
                ..Default::default()
            },
        ),

        // Define display properties
        display: MousefoodDisplay::new(geometry::Size::new(128, 64)),
    };

    // Create embedded backend with default font configuration
    let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default())?;

    // Start ratatui with our simulator backend
    let mut terminal = Terminal::new(backend)?;

    // Run an infinite loop, where widgets will be rendered
    loop {
        terminal.draw(draw)?;
    }
}

fn draw(frame: &mut Frame) {
    // Create a paragraph widget
    let text = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });

    // Place it inside a block with a title
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");

    // Render everything to the display
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
