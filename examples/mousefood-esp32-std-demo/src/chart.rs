use crate::helpers::center;
use crate::lorem::LOREM_IPSUM;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::gpio::{Gpio0, Input, PinDriver};
use esp_idf_svc::hal::task::notification::Notification;
use mousefood::prelude::*;
use mousefood::ratatui::widgets::{Axis, Block, Chart, Clear, Dataset, Paragraph, Wrap};
use std::io::Result;
use std::marker::PhantomData;

pub struct ChartApp<B: Backend> {
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
    popup: bool,
    _marker: PhantomData<B>,
}

#[derive(Clone)]
struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    const fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

impl<B: Backend> ChartApp<B> {
    pub fn new() -> Self {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();

        Self {
            signal1,
            data1,
            signal2,
            data2,
            window: [0.0, 20.0],
            popup: false,
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
                if self.popup {
                    return Ok(());
                }
                self.popup = true;
                button.enable_interrupt().unwrap();
            }
            terminal.draw(|frame| self.draw(frame))?;
            self.on_tick();
        }
    }

    fn on_tick(&mut self) {
        self.data1.drain(0..5);
        self.data1.extend(self.signal1.by_ref().take(5));

        self.data2.drain(0..10);
        self.data2.extend(self.signal2.by_ref().take(10));

        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }

    fn draw(&mut self, frame: &mut Frame) {
        let x_labels = vec![
            Span::styled(
                format!("{}", self.window[0]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{}", (self.window[0] + self.window[1]) / 2.0)),
            Span::styled(
                format!("{}", self.window[1]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let datasets = vec![
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&self.data1),
            Dataset::default()
                .name("data3")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .data(&self.data2),
        ];

        let chart = Chart::new(datasets)
            .block(Block::bordered())
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels(x_labels)
                    .bounds(self.window),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels(["-20".bold(), "0".into(), "20".bold()])
                    .bounds([-20.0, 20.0]),
            );

        let [top_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());
        frame.render_widget(chart, top_area);

        if self.popup {
            let footer = Line::raw("[S1] to change screen").centered().gray();
            frame.render_widget(footer, footer_area);

            let style = Style::default().fg(Color::Black).bg(Color::Yellow);
            let area = center(frame.area(), Constraint::Length(24), Constraint::Length(8));
            let block = Block::bordered().border_style(style).title("Popup!");
            let text = Paragraph::new(LOREM_IPSUM)
                .block(block)
                .style(style)
                .wrap(Wrap { trim: true });
            frame.render_widget(Clear, area);
            frame.render_widget(text, area);
        } else {
            let footer = Line::raw("[S1] to display popup").centered().gray();
            frame.render_widget(footer, footer_area);
        }
    }
}
