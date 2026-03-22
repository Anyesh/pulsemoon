use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Gauge, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::bordered().title(format!(" CPU: {} ", app.cpu_metrics.cpu_name));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // We need: global gauge (3 lines), sparkline (5 lines), then per-core gauges
    let core_count = app.cpu_metrics.per_core.len().min(16);

    let [global_area, sparkline_area, cores_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Fill(1),
    ])
    .areas(inner);

    // Global CPU usage gauge
    let cpu_usage = app.cpu_metrics.global_usage as f64;
    let ratio = (cpu_usage / 100.0).clamp(0.0, 1.0);
    let color = if cpu_usage < 50.0 {
        Color::Green
    } else if cpu_usage < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    let gauge = Gauge::default()
        .block(Block::bordered().title(" Global Usage "))
        .gauge_style(Style::new().fg(color).add_modifier(Modifier::BOLD))
        .ratio(ratio)
        .label(format!("{:.1}%", cpu_usage));
    frame.render_widget(gauge, global_area);

    // CPU history sparkline
    let cpu_data: Vec<u64> = app.cpu_metrics.history.iter().map(|v| *v as u64).collect();
    let sparkline = Sparkline::default()
        .block(Block::bordered().title(" CPU History "))
        .data(&cpu_data)
        .max(100)
        .style(Style::new().fg(Color::Cyan));
    frame.render_widget(sparkline, sparkline_area);

    // Per-core gauges
    if core_count == 0 {
        let msg = Paragraph::new("No per-core data available");
        frame.render_widget(msg, cores_area);
        return;
    }

    let core_block = Block::bordered().title(format!(" Per-Core ({} cores) ", app.cpu_metrics.per_core.len()));
    let core_inner = core_block.inner(cores_area);
    frame.render_widget(core_block, cores_area);

    let constraints: Vec<Constraint> = (0..core_count)
        .map(|_| Constraint::Length(2))
        .chain(std::iter::once(Constraint::Fill(1)))
        .collect();
    let core_rows = Layout::vertical(constraints).split(core_inner);

    for (i, &usage) in app.cpu_metrics.per_core.iter().take(16).enumerate() {
        let u = usage as f64;
        let r = (u / 100.0).clamp(0.0, 1.0);
        let c = if u < 50.0 {
            Color::Green
        } else if u < 80.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        let gauge = Gauge::default()
            .gauge_style(Style::new().fg(c))
            .ratio(r)
            .label(format!("Core {:>2}: {:.1}%", i, u));
        frame.render_widget(gauge, core_rows[i]);
    }
}
