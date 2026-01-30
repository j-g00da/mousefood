use core::marker::PhantomData;

use crate::button::Button;
use crate::helpers::center;
use esp_hal::delay::Delay;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, RatatuiLogo};

pub struct RatatuiLogoApp<B: Backend> {
    _marker: PhantomData<B>,
}

impl<B: Backend> RatatuiLogoApp<B> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<B>, button: &mut Button<'_>, delay: &Delay)
    where
        B::Error: 'static,
    {
        log::debug!("Running the Logo app");
        loop {
            if button.was_pressed() {
                return;
            }
            terminal.draw(|frame| self.draw(frame)).unwrap();
            delay.delay_millis(33);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [top_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());
        let content_area = center(top_area, Constraint::Length(31), Constraint::Length(8));
        let [content_block_area, ratatui_url_area, mousefood_url_area] = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(content_area);

        let block = Block::bordered()
            .padding(Padding::uniform(1))
            .border_style(Color::Yellow)
            .title("100% Mousefood™-fed rodent");
        let logo_area = block.inner(content_block_area);
        frame.render_widget(block, content_block_area);
        frame.render_widget(RatatuiLogo::small(), logo_area);
        frame.render_widget(
            "github.com/ratatui/ratatui".gray().underlined(),
            ratatui_url_area,
        );
        frame.render_widget(
            "github.com/ratatui/mousefood".gray().underlined(),
            mousefood_url_area,
        );

        let footer = Line::raw("[S1] to change screen").centered().gray();
        frame.render_widget(footer, footer_area);
    }
}
