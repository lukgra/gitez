mod chat;
mod input;
mod output;

use crate::app::App;
use ratatui::prelude::*;

struct ColorScheme {
    unfocused_window_boarder: Color,
    focused_window_boarder: Color,
    text: Color,
    command_complete: Color,
    command_failed: Color,
    error: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            unfocused_window_boarder: Color::Magenta,
            focused_window_boarder: Color::Green,
            text: Color::White,
            command_complete: Color::Green,
            command_failed: Color::DarkGray,
            error: Color::Red,
        }
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // messages + commands
            Constraint::Percentage(80), // terminal output
            Constraint::Percentage(10), // input
        ])
        .split(frame.area());

    let color_scheme = ColorScheme::default();

    chat::draw(frame, app, color_scheme, chunks[0]);
    output::draw(frame, app, chunks[1]);
    input::draw(frame, app, chunks[2]);
}
