use crate::app::{App, AppMode};
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let focused_color = Color::Green;
    let unfocused_color = Color::White;
    let input_focused = app.mode == AppMode::Input;

    let widget = Paragraph::new(app.input.value.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if input_focused {
                focused_color
            } else {
                unfocused_color
            }))
            .title("prompt (i to type, Enter to send, Esc to cancel, q to quit)"),
    );

    frame.render_widget(widget, area);

    if input_focused {
        frame.set_cursor_position(Position::new(
            area.x + app.cursor.pos as u16 + 1,
            area.y + 1,
        ));
    }
}
