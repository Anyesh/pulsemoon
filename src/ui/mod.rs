mod dashboard;
mod cpu_detail;
mod memory_detail;
mod disk_detail;
mod gpu_detail;
mod process_table;
mod port_table;
mod command_palette;
mod help;

use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph, Tabs},
    Frame,
};

use crate::app::{App, InputMode, View};

pub fn render(frame: &mut Frame, app: &mut App) {
    let [tab_area, content_area, status_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    // Tab bar
    let tab_titles = View::titles();
    let tabs = Tabs::new(tab_titles.to_vec())
        .block(Block::bordered().title(" pulsemon "))
        .select(app.view.index())
        .style(Style::new().fg(Color::White))
        .highlight_style(
            Style::new()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )
        .divider(" | ");
    frame.render_widget(tabs, tab_area);

    // Content area based on current view
    match app.view {
        View::Dashboard => dashboard::render(frame, app, content_area),
        View::CpuDetail => cpu_detail::render(frame, app, content_area),
        View::MemoryDetail => memory_detail::render(frame, app, content_area),
        View::DiskDetail => disk_detail::render(frame, app, content_area),
        View::GpuDetail => gpu_detail::render(frame, app, content_area),
        View::ProcessTable => process_table::render(frame, app, content_area),
        View::PortTable => port_table::render(frame, app, content_area),
    }

    // Status bar
    let status_text = if let Some(msg) = app.status_text() {
        format!(" {} ", msg)
    } else {
        let rate = app.tick_rate.as_millis();
        format!(
            " q:quit ?:help /:filter s:sort K:kill ::cmd +/-:rate({}ms) Tab:view 1-7:jump ",
            rate
        )
    };
    let status = Paragraph::new(status_text).style(Style::new().fg(Color::Black).bg(Color::DarkGray));
    frame.render_widget(status, status_area);

    // Overlays
    match &app.input_mode {
        InputMode::CommandPalette => command_palette::render(frame, app),
        InputMode::Filter => render_filter_bar(frame, app),
        InputMode::ConfirmKill => render_kill_confirm(frame, app),
        InputMode::Normal => {}
    }

    if app.show_help {
        help::render(frame);
    }
}

fn render_filter_bar(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let filter_area = ratatui::layout::Rect {
        x: 0,
        y: area.height.saturating_sub(3),
        width: area.width,
        height: 3,
    };

    let block = Block::bordered()
        .title(" Filter (Esc to cancel) ")
        .style(Style::new().fg(Color::Yellow));
    let input = Paragraph::new(format!("/{}", app.filter_input)).block(block);
    frame.render_widget(ratatui::widgets::Clear, filter_area);
    frame.render_widget(input, filter_area);
}

fn render_kill_confirm(frame: &mut Frame, app: &App) {
    if let Some((pid, name)) = &app.confirm_kill {
        let area = frame.area();
        let popup_width = 50.min(area.width);
        let popup_height = 5.min(area.height);
        let popup_area = ratatui::layout::Rect {
            x: (area.width - popup_width) / 2,
            y: (area.height - popup_height) / 2,
            width: popup_width,
            height: popup_height,
        };

        let text = format!("Kill {} (PID {})?\n\n[y]es / [n]o", name, pid);
        let block = Block::bordered()
            .title(" Confirm Kill ")
            .style(Style::new().fg(Color::Red));
        let popup = Paragraph::new(text).block(block);
        frame.render_widget(ratatui::widgets::Clear, popup_area);
        frame.render_widget(popup, popup_area);
    }
}
