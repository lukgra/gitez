use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    // Messages area
    let messages: Vec<Line> = app
        .messages
        .iter()
        .map(|m| Line::from(m.as_str()))
        .collect();

    let messages_widget = Paragraph::new(messages)
        .block(Block::default().borders(Borders::ALL).title("gitez"))
        .wrap(Wrap { trim: false });

    frame.render_widget(messages_widget, chunks[0]);

    // Input box
    let input_widget = Paragraph::new(app.input.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Command (press 'i' to type, Enter to send, Esc to cancel, q to quit)"),
    );

    frame.render_widget(input_widget, chunks[1]);
}
