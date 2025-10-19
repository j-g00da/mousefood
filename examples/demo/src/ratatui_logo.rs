use crate::helpers::center;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::gpio::{Gpio0, Input, PinDriver};
use esp_idf_svc::hal::task::notification::Notification;
use mousefood::prelude::*;
use mousefood::ratatui::widgets::{Block, Padding, RatatuiLogo};
use std::io::Result;
use std::marker::PhantomData;

pub struct RatatuiLogoApp<B: Backend> {
    _marker: PhantomData<B>,
}

impl<B: Backend> RatatuiLogoApp<B> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn run(
        mut self,
        terminal: &mut Terminal<B>,
        notification: &mut Notification,
        button: &mut PinDriver<Gpio0, Input>,
    ) -> Result<()> {
        button.enable_interrupt().unwrap();
        loop {
            if notification.wait(delay::NON_BLOCK).is_some() {
                return Ok(());
            }
            terminal.draw(|frame| self.draw(frame))?;
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
            "github.com/j-g00da/mousefood".gray().underlined(),
            mousefood_url_area,
        );

        let footer = Line::raw("[S1] to change screen").centered().gray();
        frame.render_widget(footer, footer_area);
    }
}
