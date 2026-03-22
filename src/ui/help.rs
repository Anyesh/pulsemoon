use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};

use crate::theme;

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let popup_width = 62u16.min(area.width);
    let popup_height = 26u16.min(area.height);

    let popup_area = Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    frame.render_widget(Clear, popup_area);

    let key_style = Style::new().fg(theme::TEXT);
    let desc_style = theme::dim_style();
    let section_style = theme::header_style();
    let blank = Line::from("");

    let lines = vec![
        blank.clone(),
        Line::from(Span::styled("  Navigation", section_style)),
        blank.clone(),
        Line::from(vec![
            Span::styled("    1-7           ", key_style),
            Span::styled("Jump to view", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    Tab / S-Tab   ", key_style),
            Span::styled("Cycle views", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    Esc           ", key_style),
            Span::styled("Back to dashboard / quit", desc_style),
        ]),
        blank.clone(),
        Line::from(Span::styled("  Tables", section_style)),
        blank.clone(),
        Line::from(vec![
            Span::styled("    j/k  \u{2191}/\u{2193}      ", key_style),
            Span::styled("Navigate rows", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    PgUp / PgDn   ", key_style),
            Span::styled("Page scroll", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    s / S         ", key_style),
            Span::styled("Cycle sort / reverse", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    /             ", key_style),
            Span::styled("Filter", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    K / Del       ", key_style),
            Span::styled("Kill selected", desc_style),
        ]),
        blank.clone(),
        Line::from(Span::styled("  Actions", section_style)),
        blank.clone(),
        Line::from(vec![
            Span::styled("    :             ", key_style),
            Span::styled("Command palette", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    + / -         ", key_style),
            Span::styled("Adjust refresh rate", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    ?             ", key_style),
            Span::styled("Toggle this help", desc_style),
        ]),
        Line::from(vec![
            Span::styled("    q             ", key_style),
            Span::styled("Quit", desc_style),
        ]),
    ];

    let title = Line::from(vec![
        Span::styled(" Help ", theme::title_style()),
        Span::styled("\u{2014} Keybindings ", theme::dim_style()),
    ]);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::border_style())
        .title(title);

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}
