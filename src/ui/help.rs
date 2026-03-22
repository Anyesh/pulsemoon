use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let popup_width = 60u16.min(area.width);
    let popup_height = 22u16.min(area.height);

    let popup_area = Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    frame.render_widget(Clear, popup_area);

    let help_text = "\
Navigation
  1-7         Jump to view
  Tab/S-Tab   Cycle views
  Esc         Back to dashboard / quit

Tables
  j/k \u{2191}/\u{2193}    Navigate rows
  PgUp/PgDn   Page scroll
  s/S         Cycle sort / reverse
  /           Filter
  K/Del       Kill selected

Actions
  :           Command palette
  +/-         Adjust refresh rate
  ?           Toggle this help
  q           Quit

Commands
  :kill <pid>        Kill by PID
  :kill-port <port>  Kill by port
  :sort <column>     Sort table
  :rate <ms>         Set refresh rate
  :filter <text>     Filter table";

    let block = Block::bordered()
        .title(" Help \u{2014} Keybindings ")
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(help_text).block(block);
    frame.render_widget(paragraph, popup_area);
}
