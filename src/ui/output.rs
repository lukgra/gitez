use crate::app::App;
use crate::ui::ColorScheme;
use ratatui::{prelude::*, widgets::*};

/// Draw output window
pub fn draw(frame: &mut Frame, app: &App, color_scheme: &ColorScheme, area: Rect) {
    let output_lines: Vec<Line> = app
        .output
        .lines
        .iter()
        .map(|o| Line::from(o.as_str()).style(Style::default().fg(color_scheme.output_text)))
        .collect();

    let widget = Paragraph::new(output_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(color_scheme.unfocused_window_boarder))
                .title("Output"),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(widget, area);
}
