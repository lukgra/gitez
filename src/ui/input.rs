use crate::app::{App, AppMode};
use crate::ui::ColorScheme;
use ratatui::{prelude::*, widgets::*};

/// Draw input window
pub fn draw(frame: &mut Frame, app: &App, color_scheme: &ColorScheme, area: Rect) {
    let input_focused = app.mode == AppMode::Input;

    let widget = Paragraph::new(app.input.value.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if input_focused {
                color_scheme.focused_window_boarder
            } else {
                color_scheme.unfocused_window_boarder
            }))
            .title("Prompt (i to type, Enter to send, Esc to cancel, q to quit)"),
    );

    frame.render_widget(widget, area);

    if input_focused {
        frame.set_cursor_position(Position::new(
            area.x + app.cursor.pos as u16 + 1,
            area.y + 1,
        ));
    }
}
