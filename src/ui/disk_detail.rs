use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Gauge, Paragraph},
    Frame,
};

use crate::app::App;
use crate::types::format_bytes;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::bordered().title(" Disk Detail ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.disk_metrics.is_empty() {
        let msg = Paragraph::new("No disks detected");
        frame.render_widget(msg, inner);
        return;
    }

    // Each disk gets 3 lines: 1 for label text, 1 for gauge, 1 for spacing
    let constraints: Vec<Constraint> = app
        .disk_metrics
        .iter()
        .flat_map(|_| [Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
        .chain(std::iter::once(Constraint::Fill(1)))
        .collect();
    let rows = Layout::vertical(constraints).split(inner);

    for (i, disk) in app.disk_metrics.iter().enumerate() {
        let base = i * 3;
        if base + 1 >= rows.len() {
            break;
        }

        let total = disk.total as f64;
        let ratio = if total > 0.0 {
            (disk.used as f64 / total).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let pct = ratio * 100.0;

        let color = if pct < 50.0 {
            Color::Green
        } else if pct < 80.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        // Label line: mount point and filesystem type
        let label_text = format!(
            "{} [{}] - {}",
            disk.mount_point, disk.fs_type, disk.name
        );
        let label = Paragraph::new(label_text)
            .style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        frame.render_widget(label, rows[base]);

        // Gauge line
        let gauge = Gauge::default()
            .gauge_style(Style::new().fg(color))
            .ratio(ratio)
            .label(format!(
                "{} / {} ({:.1}%)",
                format_bytes(disk.used),
                format_bytes(disk.total),
                pct
            ));
        frame.render_widget(gauge, rows[base + 1]);
    }
}
