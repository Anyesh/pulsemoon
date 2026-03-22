use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
    Frame,
};

use crate::app::App;
use crate::types::PortSortBy;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let filtered = app.filtered_ports();
    let count = filtered.len();

    let arrow = if app.port_sort_asc { " \u{25b2}" } else { " \u{25bc}" };

    let protocol_header = if app.port_sort_by == PortSortBy::Protocol {
        format!("Protocol{}", arrow)
    } else {
        "Protocol".to_string()
    };
    let port_header = if app.port_sort_by == PortSortBy::Port {
        format!("Port{}", arrow)
    } else {
        "Port".to_string()
    };
    let state_header = if app.port_sort_by == PortSortBy::State {
        format!("State{}", arrow)
    } else {
        "State".to_string()
    };
    let pid_header = if app.port_sort_by == PortSortBy::Pid {
        format!("PID{}", arrow)
    } else {
        "PID".to_string()
    };

    let header = Row::new(vec![
        Cell::from(protocol_header),
        Cell::from("Local Address".to_string()),
        Cell::from(port_header),
        Cell::from("Remote Address".to_string()),
        Cell::from(state_header),
        Cell::from(pid_header),
        Cell::from("Process".to_string()),
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
                Cell::from(p.protocol.clone()),
                Cell::from(p.local_addr.clone()),
                Cell::from(p.local_port.to_string()),
                Cell::from(p.remote_addr.clone()),
                Cell::from(p.state.clone()),
                Cell::from(p.pid.map(|pid| pid.to_string()).unwrap_or_else(|| "-".into())),
                Cell::from(p.process_name.clone()),
            ])
        })
        .collect();

    let title = format!(" Ports ({}) ", count);
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
            Constraint::Length(20),
            Constraint::Length(12),
            Constraint::Length(8),
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

    frame.render_stateful_widget(table, area, &mut app.port_table_state);
}
