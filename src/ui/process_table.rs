use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
    Frame,
};

use crate::app::App;
use crate::types::{format_bytes, ProcessSortBy};

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let filtered = app.filtered_processes();
    let count = filtered.len();

    let arrow = if app.process_sort_asc { " \u{25b2}" } else { " \u{25bc}" };

    let pid_header = if app.process_sort_by == ProcessSortBy::Pid {
        format!("PID{}", arrow)
    } else {
        "PID".to_string()
    };
    let name_header = if app.process_sort_by == ProcessSortBy::Name {
        format!("Name{}", arrow)
    } else {
        "Name".to_string()
    };
    let cpu_header = if app.process_sort_by == ProcessSortBy::Cpu {
        format!("CPU%{}", arrow)
    } else {
        "CPU%".to_string()
    };
    let mem_header = if app.process_sort_by == ProcessSortBy::Memory {
        format!("Memory{}", arrow)
    } else {
        "Memory".to_string()
    };

    let header = Row::new(vec![
        Cell::from(pid_header),
        Cell::from(name_header),
        Cell::from(cpu_header),
        Cell::from(mem_header),
        Cell::from("Status".to_string()),
        Cell::from("Command".to_string()),
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = filtered
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.1}%", p.cpu_usage)),
                Cell::from(format_bytes(p.memory)),
                Cell::from(p.status.clone()),
                Cell::from(p.command.clone()),
            ])
        })
        .collect();

    let title = format!(" Processes ({}) ", count);
    let filter_suffix = if !app.filter_input.is_empty() {
        format!(" [filter: {}] ", app.filter_input)
    } else {
        String::new()
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(20),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Fill(1),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .title(title)
            .title_bottom(filter_suffix),
    )
    .row_highlight_style(
        Style::default()
            .bg(Color::Cyan)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(table, area, &mut app.process_table_state);
}
