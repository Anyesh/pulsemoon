use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Gauge, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;
use crate::types::format_bytes;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::bordered().title(" Memory Detail ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [ram_area, swap_area, sparkline_area, totals_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Fill(1),
    ])
    .areas(inner);

    // RAM usage gauge
    let mem_total = app.memory_metrics.total as f64;
    let mem_ratio = if mem_total > 0.0 {
        (app.memory_metrics.used as f64 / mem_total).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let mem_pct = mem_ratio * 100.0;
    let mem_color = if mem_pct < 50.0 {
        Color::Green
    } else if mem_pct < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    let ram_gauge = Gauge::default()
        .block(Block::bordered().title(" RAM "))
        .gauge_style(Style::new().fg(mem_color).add_modifier(Modifier::BOLD))
        .ratio(mem_ratio)
        .label(format!(
            "{} / {} ({:.1}%)",
            format_bytes(app.memory_metrics.used),
            format_bytes(app.memory_metrics.total),
            mem_pct
        ));
    frame.render_widget(ram_gauge, ram_area);

    // Swap usage gauge
    let swap_total = app.memory_metrics.swap_total as f64;
    let swap_ratio = if swap_total > 0.0 {
        (app.memory_metrics.swap_used as f64 / swap_total).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let swap_pct = swap_ratio * 100.0;
    let swap_color = if swap_pct < 50.0 {
        Color::Green
    } else if swap_pct < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    let swap_gauge = Gauge::default()
        .block(Block::bordered().title(" Swap "))
        .gauge_style(Style::new().fg(swap_color).add_modifier(Modifier::BOLD))
        .ratio(swap_ratio)
        .label(format!(
            "{} / {} ({:.1}%)",
            format_bytes(app.memory_metrics.swap_used),
            format_bytes(app.memory_metrics.swap_total),
            swap_pct
        ));
    frame.render_widget(swap_gauge, swap_area);

    // Memory history sparkline
    let mem_data: Vec<u64> = app.memory_metrics.history.iter().map(|v| *v as u64).collect();
    let sparkline = Sparkline::default()
        .block(Block::bordered().title(" Memory History "))
        .data(&mem_data)
        .max(100)
        .style(Style::new().fg(Color::Magenta));
    frame.render_widget(sparkline, sparkline_area);

    // System totals text
    let totals_block = Block::bordered().title(" System Totals ");
    let totals_text = format!(
        "Total RAM:  {}\nUsed RAM:   {}\nFree RAM:   {}\n\nTotal Swap: {}\nUsed Swap:  {}\nFree Swap:  {}",
        format_bytes(app.memory_metrics.total),
        format_bytes(app.memory_metrics.used),
        format_bytes(app.memory_metrics.total.saturating_sub(app.memory_metrics.used)),
        format_bytes(app.memory_metrics.swap_total),
        format_bytes(app.memory_metrics.swap_used),
        format_bytes(app.memory_metrics.swap_total.saturating_sub(app.memory_metrics.swap_used)),
    );
    let totals = Paragraph::new(totals_text)
        .block(totals_block)
        .style(Style::new().fg(Color::White));
    frame.render_widget(totals, totals_area);
}
