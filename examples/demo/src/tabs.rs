use crate::lorem::LOREM_IPSUM;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::gpio::{Gpio0, Input, PinDriver};
use esp_idf_svc::hal::task::notification::Notification;
use mousefood::prelude::*;
use mousefood::ratatui::style::Style;
use mousefood::ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use mousefood::ratatui::widgets::{Bar, BarChart, BarGroup, Block, Padding, Paragraph, Tabs, Wrap};
use rand::{Rng, rng};
use std::marker::PhantomData;
use time::{Date, Month};

pub struct TabsApp<B: Backend> {
    selected_tab: usize,
    temperatures: Vec<u8>,
    _marker: PhantomData<B>,
}

impl<B: Backend> TabsApp<B> {
    pub fn new() -> Self {
        let mut rng = rng();
        let temperatures = (0..6).map(|_| rng.random_range(50..90)).collect();
        Self {
            selected_tab: 0,
            temperatures,
            _marker: PhantomData,
        }
    }
    pub fn run(
        mut self,
        terminal: &mut Terminal<B>,
        notification: &mut Notification,
        button: &mut PinDriver<Gpio0, Input>,
    ) -> std::io::Result<()> {
        button.enable_interrupt().unwrap();
        loop {
            if notification.wait(delay::NON_BLOCK).is_some() {
                if self.selected_tab == 2 {
                    return Ok(());
                }
                self.selected_tab += 1;
                button.enable_interrupt().unwrap();
            }
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }
    }
}

impl<B: Backend> Widget for &TabsApp<B> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        self.render_tabs(header_area, buf);
        let block = Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(Color::Yellow);
        match self.selected_tab {
            0 => {
                Paragraph::new(LOREM_IPSUM)
                    .wrap(Wrap { trim: true })
                    .block(block)
                    .render(inner_area, buf);
                render_footer("[S1] to change tab", footer_area, buf);
            }
            1 => {
                let default_style = Style::default()
                    .bg(Color::Rgb(50, 50, 50))
                    .fg(Color::Yellow);
                let list = CalendarEventStore::today(Style::default().bg(Color::Blue));
                Monthly::new(
                    Date::from_calendar_date(2025, Month::May, 23).unwrap(),
                    list,
                )
                .show_month_header(Style::default().fg(Color::Yellow))
                .default_style(default_style)
                .block(block)
                .render(inner_area, buf);
                render_footer("[S1] to change tab", footer_area, buf);
            }
            2 => {
                vertical_barchart(&self.temperatures)
                    .block(block)
                    .render(inner_area, buf);
                render_footer("[S1] to change screen", footer_area, buf);
            }
            _ => {}
        }
    }
}

impl<B: Backend> TabsApp<B> {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = ["[Paragraph]", "[Calendar]", "[Barchart]"];
        Tabs::new(titles)
            .style(Style::new().bg(Color::Black).fg(Color::Yellow))
            .highlight_style(Style::new().bg(Color::Yellow).fg(Color::Black))
            .select(self.selected_tab)
            .render(area, buf);
    }
}

impl<B: Backend> Default for TabsApp<B> {
    fn default() -> Self {
        Self::new()
    }
}

fn vertical_barchart(temperatures: &[u8]) -> BarChart {
    let bars: Vec<Bar> = temperatures
        .iter()
        .enumerate()
        .map(|(hour, value)| vertical_bar(hour, value))
        .collect();
    let title = Line::from("Weather (Vertical)").centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title))
        .bar_width(5)
}

fn vertical_bar(hour: usize, temperature: &u8) -> Bar {
    Bar::default()
        .value(u64::from(*temperature))
        .label(Line::from(format!("{hour:>02}:00")))
        .text_value(format!("{temperature:>3}°"))
        .style(temperature_style(*temperature))
        .value_style(label_style(*temperature))
}

fn temperature_style(value: u8) -> Style {
    let green = (255.0 * (1.0 - f64::from(value - 50) / 40.0)) as u8;
    let color = Color::Rgb(255, green, 0);
    Style::new().fg(color)
}

fn label_style(value: u8) -> Style {
    let green = (255.0 * (1.0 - f64::from(value - 50) / 40.0)) as u8;
    let color = Color::Rgb(255, green, 0);
    Style::new().bg(color).fg(Color::Black)
}

fn render_footer(content: &str, area: Rect, buf: &mut Buffer) {
    Line::raw(content).centered().gray().render(area, buf);
}
