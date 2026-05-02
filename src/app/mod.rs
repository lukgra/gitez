mod chat;
mod cursor;
mod input;
mod output;

pub use chat::{Chat, ChatMessage};
pub use cursor::Cursor;
pub use input::Input;
pub use output::Output;

use crate::ollama;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time;
use tokio::sync::mpsc;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Input,
    Exit,
    Review,
}

/// App controler
pub struct App {
    pub mode: AppMode,
    pub chat: Chat,
    pub input: Input,
    pub output: Output,
    pub cursor: Cursor,
    pub tx: mpsc::Sender<ChatMessage>,
    pub rx: mpsc::Receiver<ChatMessage>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        let mut app = Self {
            mode: AppMode::Normal,
            input: Input::new(),
            chat: Chat::new(),
            output: Output::new(),
            cursor: Cursor::new(),
            tx,
            rx,
        };
        app.chat
            .push("Welcome to gitez! Press 'i' to start typing.".to_string());
        app
    }

    /// Main update loop, polls for responses and handles events
    pub async fn update(&mut self) -> anyhow::Result<()> {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                ChatMessage::Commands(cmds) => {
                    self.chat.load_commands(cmds);
                    self.cursor.move_end(self.chat.current_command_value.len());
                    self.mode = AppMode::Review;
                }
                ChatMessage::Error(e) => self.chat.set_error(e),
            }
        }

        if event::poll(time::Duration::from_millis(100))? {
            self.process_event().await?;
        }

        Ok(())
    }

    /// Processes app event (user input)
    pub async fn process_event(&mut self) -> anyhow::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.mode {
                AppMode::Normal => self.handle_normal_mode(&key),
                AppMode::Input => self.handle_input_mode(&key),
                AppMode::Review => self.handle_review_mode(&key),
                _ => (),
            }
        }
        Ok(())
    }

    fn handle_normal_mode(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.mode = AppMode::Exit,
            KeyCode::Char('i') => {
                self.mode = AppMode::Input;
                self.cursor.move_end(self.input.value.len());
            }
            _ => (),
        }
    }

    fn handle_input_mode(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Enter => self.handle_command_submit(),
            KeyCode::Char(c) => self.input.insert_char(&mut self.cursor, c),
            KeyCode::Backspace => self.input.backspace(&mut self.cursor),
            KeyCode::Left => self.cursor.move_left(),
            KeyCode::Right => self.cursor.move_right(self.input.value.len()),
            KeyCode::Home => self.cursor.move_home(),
            KeyCode::End => self.cursor.move_end(self.input.value.len()),
            KeyCode::Esc => {
                self.input.clear(&mut self.cursor);
                self.mode = AppMode::Normal;
            }
            _ => (),
        }
    }

    fn handle_review_mode(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Enter => self.execute_current_command(),
            KeyCode::Char(c) => {
                self.chat.current_command_value.insert(self.cursor.pos, c);
                self.cursor.pos += 1;
            }
            KeyCode::Backspace => {
                if self.cursor.pos > 0 {
                    self.chat.current_command_value.remove(self.cursor.pos - 1);
                    self.cursor.move_left();
                }
            }
            KeyCode::Left => self.cursor.move_left(),
            KeyCode::Right => self
                .cursor
                .move_right(self.chat.current_command_value.len()),
            KeyCode::Home => self.cursor.move_home(),
            KeyCode::End => self.cursor.move_end(self.chat.current_command_value.len()),
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                self.chat.pending_commands.clear();
                self.chat.current_command_value.clear();
                self.cursor.reset();
            }
            _ => (),
        }
    }

    fn handle_command_submit(&mut self) {
        let tx = self.tx.clone();
        let cmd = self.input.drain(&mut self.cursor);

        self.chat.clear();
        self.output.clear();
        self.chat.clear_error();
        self.chat.push(format!("> {}", cmd));
        self.mode = AppMode::Normal;

        tokio::spawn(async move {
            match ollama::ask_ollama(&cmd).await {
                Ok(response) => {
                    let cmds: Vec<String> = response
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| line.to_string())
                        .collect();
                    let _ = tx.send(ChatMessage::Commands(cmds)).await;
                }
                Err(e) => {
                    let _ = tx.send(ChatMessage::Error(format!("Error: {}", e))).await;
                }
            }
        });
    }

    fn execute_current_command(&mut self) {
        let cmd = self.chat.current_command_value.clone();
        let result = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output();

        match result {
            Ok(out) => {
                self.chat.executed_commands.push(cmd.clone());
                let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();

                if !stdout.is_empty() {
                    self.output.push(stdout);
                }

                if !stderr.is_empty() && !out.status.success() {
                    self.output.push(format!("✗ {}", stderr));
                }
            }
            Err(e) => self.output.push(format!("✗ Error: {}", e)),
        }

        if self.chat.next_command() {
            self.cursor.move_end(self.chat.current_command_value.len());
        } else {
            self.mode = AppMode::Normal;
            self.chat.pending_commands.clear();
            self.chat.current_command_value.clear();
            self.cursor.reset();
        }
    }
}
