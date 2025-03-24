extern crate mousefood;

use embedded_graphics_simulator::SimulatorDisplay;
use mousefood::embedded_graphics::geometry;
use mousefood::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};

fn main() -> Result<(), std::io::Error> {
    let mut display = SimulatorDisplay::<Bgr565>::new(geometry::Size::new(128, 64));
    let backend: EmbeddedBackend<SimulatorDisplay<Bgr565>, _, { 128 * 64 }> =
        EmbeddedBackend::new(&mut display);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(draw)?;
    }
}

fn draw(frame: &mut Frame) {
    let text1 = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text1.dark_gray()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
