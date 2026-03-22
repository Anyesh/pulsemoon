use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Gauge, Paragraph, Row, Sparkline, Table},
    Frame,
};

use crate::app::App;
use crate::types::format_bytes;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let [top_area, sparkline_area, disk_gpu_area, process_area] = Layout::vertical([
        Constraint::Length(7),
        Constraint::Length(5),
        Constraint::Length(7),
        Constraint::Fill(1),
    ])
    .areas(area);

    render_top_gauges(frame, app, top_area);
    render_sparklines(frame, app, sparkline_area);
    render_disk_gpu(frame, app, disk_gpu_area);
    render_mini_process_table(frame, app, process_area);
}

fn cpu_color(usage: f64) -> Color {
    if usage < 50.0 {
        Color::Green
    } else if usage < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn render_top_gauges(frame: &mut Frame, app: &App, area: Rect) {
    let [cpu_area, mem_area] = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .areas(area);

    // CPU gauge
    let cpu_usage = app.cpu_metrics.global_usage as f64;
    let cpu_ratio = (cpu_usage / 100.0).clamp(0.0, 1.0);
    let cpu_gauge = Gauge::default()
        .block(Block::bordered().title(" CPU "))
        .gauge_style(Style::new().fg(cpu_color(cpu_usage)).add_modifier(Modifier::BOLD))
        .ratio(cpu_ratio)
        .label(format!("{:.1}%", cpu_usage));
    frame.render_widget(cpu_gauge, cpu_area);

    // Memory gauge
    let mem_total = app.memory_metrics.total as f64;
    let mem_ratio = if mem_total > 0.0 {
        (app.memory_metrics.used as f64 / mem_total).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let mem_pct = mem_ratio * 100.0;
    let mem_gauge = Gauge::default()
        .block(Block::bordered().title(" Memory "))
        .gauge_style(Style::new().fg(cpu_color(mem_pct)).add_modifier(Modifier::BOLD))
        .ratio(mem_ratio)
        .label(format!(
            "{} / {} ({:.1}%)",
            format_bytes(app.memory_metrics.used),
            format_bytes(app.memory_metrics.total),
            mem_pct
        ));
    frame.render_widget(mem_gauge, mem_area);
}

fn render_sparklines(frame: &mut Frame, app: &App, area: Rect) {
    let [cpu_area, mem_area] = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .areas(area);

    let cpu_data: Vec<u64> = app.cpu_metrics.history.iter().map(|v| *v as u64).collect();
    let cpu_sparkline = Sparkline::default()
        .block(Block::bordered().title(" CPU History "))
        .data(&cpu_data)
        .max(100)
        .style(Style::new().fg(Color::Cyan));
    frame.render_widget(cpu_sparkline, cpu_area);

    let mem_data: Vec<u64> = app.memory_metrics.history.iter().map(|v| *v as u64).collect();
    let mem_sparkline = Sparkline::default()
        .block(Block::bordered().title(" Memory History "))
        .data(&mem_data)
        .max(100)
        .style(Style::new().fg(Color::Magenta));
    frame.render_widget(mem_sparkline, mem_area);
}

fn render_disk_gpu(frame: &mut Frame, app: &App, area: Rect) {
    let [disk_area, gpu_area] = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .areas(area);

    // Disk: show first 2 disks
    let disk_block = Block::bordered().title(" Disks ");
    let disk_inner = disk_block.inner(disk_area);
    frame.render_widget(disk_block, disk_area);

    let disk_count = app.disk_metrics.len().min(2);
    if disk_count > 0 {
        let constraints: Vec<Constraint> = (0..disk_count)
            .map(|_| Constraint::Length(1))
            .chain(std::iter::once(Constraint::Fill(1)))
            .collect();
        let disk_rows = Layout::vertical(constraints).split(disk_inner);

        for (i, disk) in app.disk_metrics.iter().take(2).enumerate() {
            let total = disk.total as f64;
            let ratio = if total > 0.0 {
                (disk.used as f64 / total).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let label = format!(
                "{}: {} / {}",
                disk.name,
                format_bytes(disk.used),
                format_bytes(disk.total)
            );
            let gauge = Gauge::default()
                .gauge_style(Style::new().fg(Color::Blue))
                .ratio(ratio)
                .label(label);
            frame.render_widget(gauge, disk_rows[i]);
        }
    } else {
        let msg = Paragraph::new("No disks detected");
        frame.render_widget(msg, disk_inner);
    }

    // GPU
    let gpu_block = Block::bordered().title(" GPU ");
    let gpu_inner = gpu_block.inner(gpu_area);
    frame.render_widget(gpu_block, gpu_area);

    if app.gpu_metrics.is_empty() {
        let msg = Paragraph::new("No GPU detected");
        frame.render_widget(msg, gpu_inner);
    } else {
        let gpu = &app.gpu_metrics[0];
        let utilization = gpu.utilization.unwrap_or(0.0) as f64;
        let ratio = (utilization / 100.0).clamp(0.0, 1.0);

        let [name_area, gauge_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(gpu_inner);

        let name_text = Paragraph::new(gpu.name.clone())
            .style(Style::new().fg(Color::Cyan));
        frame.render_widget(name_text, name_area);

        let gpu_gauge = Gauge::default()
            .gauge_style(Style::new().fg(Color::Green))
            .ratio(ratio)
            .label(format!("Util: {:.1}%", utilization));
        frame.render_widget(gpu_gauge, gauge_area);
    }
}

fn render_mini_process_table(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("PID"),
        Cell::from("Name"),
        Cell::from("CPU%"),
        Cell::from("Memory"),
    ])
    .style(
        Style::new()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .processes
        .iter()
        .take(5)
        .map(|p| {
            Row::new(vec![
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.1}", p.cpu_usage)),
                Cell::from(format_bytes(p.memory)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(8),
        Constraint::Fill(1),
        Constraint::Length(8),
        Constraint::Length(12),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::bordered().title(" Top Processes "));

    frame.render_widget(table, area);
}
