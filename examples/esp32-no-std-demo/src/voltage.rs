use core::marker::PhantomData;

use crate::button::Button;
use alloc::format;
use esp_hal::delay::Delay;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Padding, Paragraph};

pub struct VoltageApp<B: Backend> {
    _marker: PhantomData<B>,
}

impl<B: Backend> VoltageApp<B> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn run<F>(
        &mut self,
        terminal: &mut Terminal<B>,
        button: &mut Button<'_>,
        delay: &Delay,
        read_voltage: &mut F,
    ) where
        F: FnMut() -> Option<u16>,
        B::Error: 'static,
    {
        log::debug!("Running the Voltage app");
        loop {
            if button.was_pressed() {
                return;
            }
            if let Some(voltage) = read_voltage() {
                terminal
                    .draw(|frame| self.draw(frame, 2 * voltage))
                    .unwrap();
            }
            delay.delay_millis(33);
        }
    }

    fn draw(&mut self, frame: &mut Frame, voltage: u16) {
        let [content_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::vertical(1))
            .title("Battery voltage")
            .border_style(Style::new().yellow());
        let inner_area = block.inner(content_area);
        frame.render_widget(block, content_area);

        let voltage_text = format!("{:.2}V", voltage as f32 / 1000.0);
        let voltage_line = Line::from(voltage_text)
            .centered()
            .style(Style::new().blue());
        frame.render_widget(Paragraph::new(voltage_line), inner_area);

        let footer = Line::raw("[S1] to change screen").centered().gray();
        frame.render_widget(footer, footer_area);
    }
}
