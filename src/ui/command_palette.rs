use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let height = if app.command_error.is_some() { 4 } else { 3 };
    let palette_area = Rect {
        x: 0,
        y: area.height.saturating_sub(height),
        width: area.width,
        height: height.min(area.height),
    };

    frame.render_widget(Clear, palette_area);

    let block = Block::bordered()
        .title(" Command (Esc to cancel) ")
        .style(Style::default().fg(Color::Yellow));

    let inner = block.inner(palette_area);
    frame.render_widget(block, palette_area);

    if let Some(ref err) = app.command_error {
        let [input_area, error_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(inner);

        let input = Paragraph::new(format!(":{}", app.command_input));
        frame.render_widget(input, input_area);

        let error = Paragraph::new(err.clone()).style(Style::default().fg(Color::Red));
        frame.render_widget(error, error_area);
    } else {
        let input = Paragraph::new(format!(":{}", app.command_input));
        frame.render_widget(input, inner);
    }
}
