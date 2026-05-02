use crate::app::{App, AppMode};
use crate::ui::ColorScheme;
use ratatui::{prelude::*, widgets::*};

/// Draw chat window
pub fn draw(frame: &mut Frame, app: &App, color_scheme: ColorScheme, area: Rect) {
    let review_focused = app.mode == AppMode::Review;

    let all_lines: Vec<Line> = if let Some(error) = &app.chat.error {
        vec![Line::from(error.as_str()).style(Style::default().fg(color_scheme.error))]
    } else {
        let mut lines: Vec<Line> = app
            .chat
            .messages
            .iter()
            .map(|m| Line::from(m.as_str()).style(Style::default().fg(color_scheme.text)))
            .collect();

        for cmd in &app.chat.executed_commands {
            lines.push(
                Line::from(format!("✓ {}", cmd))
                    .style(Style::default().fg(color_scheme.command_complete)),
            );
        }

        for (i, cmd) in app.chat.pending_commands.iter().enumerate() {
            if i < app.chat.current_command_index {
                continue;
            }
            let is_active = i == app.chat.current_command_index;
            let display = if is_active {
                app.chat.current_command_value.as_str()
            } else {
                cmd.as_str()
            };
            let style = if is_active {
                Style::default()
                    .fg(color_scheme.command_complete)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color_scheme.command_failed)
            };
            lines.push(Line::from(display).style(style));
        }

        lines
    };

    let widget = Paragraph::new(all_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if review_focused {
                    color_scheme.focused_window_boarder
                } else {
                    color_scheme.unfocused_window_boarder
                }))
                .title("gitez"),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(widget, area);

    if app.mode == AppMode::Review {
        let cmd_line_y =
            area.y + app.chat.messages.len() as u16 + app.chat.executed_commands.len() as u16 + 1;

        frame.set_cursor_position(Position::new(
            area.x + app.cursor.pos as u16 + 1,
            cmd_line_y,
        ));
    }
}
