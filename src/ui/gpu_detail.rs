use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Gauge, Paragraph},
    Frame,
};

use crate::app::App;
use crate::types::format_bytes;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::bordered().title(" GPU Detail ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.gpu_metrics.is_empty() {
        let msg = Paragraph::new("No supported GPU detected")
            .style(Style::new().fg(Color::DarkGray));
        frame.render_widget(msg, inner);
        return;
    }

    // Each GPU needs: name(1) + util gauge(1) + vram gauge(1) + info line(1) + spacing(1) = 5 lines
    let constraints: Vec<Constraint> = app
        .gpu_metrics
        .iter()
        .map(|_| Constraint::Length(9))
        .chain(std::iter::once(Constraint::Fill(1)))
        .collect();
    let gpu_sections = Layout::vertical(constraints).split(inner);

    for (i, gpu) in app.gpu_metrics.iter().enumerate() {
        if i >= gpu_sections.len() - 1 {
            break;
        }
        render_single_gpu(frame, gpu, gpu_sections[i]);
    }
}

fn render_single_gpu(frame: &mut Frame, gpu: &crate::types::GpuMetrics, area: Rect) {
    let [name_area, util_area, vram_area, temp_area, power_area, fan_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(area);

    // GPU name
    let name = Paragraph::new(gpu.name.clone())
        .style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(name, name_area);

    // Utilization gauge
    let util = gpu.utilization.unwrap_or(0.0) as f64;
    let util_ratio = (util / 100.0).clamp(0.0, 1.0);
    let util_color = if util < 50.0 {
        Color::Green
    } else if util < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    };
    let util_label = match gpu.utilization {
        Some(u) => format!("Utilization: {:.1}%", u),
        None => "Utilization: N/A".to_string(),
    };
    let util_gauge = Gauge::default()
        .gauge_style(Style::new().fg(util_color))
        .ratio(util_ratio)
        .label(util_label);
    frame.render_widget(util_gauge, util_area);

    // VRAM gauge
    let (vram_ratio, vram_label) = match (gpu.memory_used, gpu.memory_total) {
        (Some(used), Some(total)) if total > 0 => {
            let ratio = (used as f64 / total as f64).clamp(0.0, 1.0);
            let label = format!(
                "VRAM: {} / {} ({:.1}%)",
                format_bytes(used),
                format_bytes(total),
                ratio * 100.0
            );
            (ratio, label)
        }
        _ => (0.0, "VRAM: N/A".to_string()),
    };
    let vram_color = if vram_ratio < 0.5 {
        Color::Green
    } else if vram_ratio < 0.8 {
        Color::Yellow
    } else {
        Color::Red
    };
    let vram_gauge = Gauge::default()
        .gauge_style(Style::new().fg(vram_color))
        .ratio(vram_ratio)
        .label(vram_label);
    frame.render_widget(vram_gauge, vram_area);

    // Temperature
    let temp_text = match gpu.temperature {
        Some(t) => format!("Temperature: {:.0} C", t),
        None => "Temperature: N/A".to_string(),
    };
    let temp = Paragraph::new(temp_text).style(Style::new().fg(Color::White));
    frame.render_widget(temp, temp_area);

    // Power
    let power_text = match (gpu.power_usage, gpu.power_limit) {
        (Some(usage), Some(limit)) => format!("Power: {:.1}W / {:.1}W", usage, limit),
        (Some(usage), None) => format!("Power: {:.1}W / N/A", usage),
        (None, Some(limit)) => format!("Power: N/A / {:.1}W", limit),
        (None, None) => "Power: N/A".to_string(),
    };
    let power = Paragraph::new(power_text).style(Style::new().fg(Color::White));
    frame.render_widget(power, power_area);

    // Fan speed
    let fan_text = match gpu.fan_speed {
        Some(speed) => format!("Fan Speed: {}%", speed),
        None => "Fan Speed: N/A".to_string(),
    };
    let fan = Paragraph::new(fan_text).style(Style::new().fg(Color::White));
    frame.render_widget(fan, fan_area);
}
