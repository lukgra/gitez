use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let output_lines: Vec<Line> = app
        .output
        .lines
        .iter()
        .map(|o| Line::from(o.as_str()).style(Style::default().fg(Color::Yellow)))
        .collect();

    let widget = Paragraph::new(output_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White))
                .title("output"),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(widget, area);
}
