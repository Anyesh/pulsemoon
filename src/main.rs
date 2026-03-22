use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let title = Paragraph::new("  pulsemon — System Monitor")
        .style(Style::new().fg(Color::Cyan))
        .block(Block::bordered());
    frame.render_widget(title, header);

    let body_text = Paragraph::new("Initializing collectors...")
        .block(Block::bordered().title("Dashboard"));
    frame.render_widget(body_text, body);

    let status = Paragraph::new(" q: quit | ?: help | :: command palette")
        .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(status, footer);
}
