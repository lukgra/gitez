use crate::ollama;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time;
use tokio::sync::mpsc;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Input,
    Exit,
}

pub struct App {
    pub input: String,
    pub messages: Vec<String>,
    pub mode: AppMode,
    pub tx: mpsc::Sender<String>,
    pub rx: mpsc::Receiver<String>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        let mut app = Self {
            input: String::new(),
            messages: Vec::new(),
            mode: AppMode::Normal,
            tx,
            rx,
        };
        app.push_message("Welcome to gitez! Press 'i' to start typing.".to_string());
        app
    }

    pub fn push_message(&mut self, msg: String) {
        self.messages.push(msg);
    }

    /// Main update loop, polls for responses and handles events
    pub async fn update(&mut self) -> anyhow::Result<()> {
        while let Ok(msg) = self.rx.try_recv() {
            self.push_message(msg);
        }

        if event::poll(time::Duration::from_millis(100))? {
            self.process_event().await?;
        }

        Ok(())
    }

    pub async fn process_event(&mut self) -> anyhow::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.mode {
                AppMode::Normal => self.handle_normal_mode(&key),
                AppMode::Input => match key.code {
                    KeyCode::Esc => {
                        self.mode = AppMode::Normal;
                        self.input.clear();
                    }
                    KeyCode::Enter => {
                        let cmd = self.input.drain(..).collect::<String>();
                        self.push_message(format!("> {}", cmd));
                        self.mode = AppMode::Normal;

                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            match ollama::ask_ollama(&cmd).await {
                                Ok(response) => {
                                    let _ = tx.send(response).await;
                                }
                                Err(e) => {
                                    let _ = tx.send(format!("Error: {}", e));
                                }
                            }
                        });
                    }
                    KeyCode::Char(c) => self.input.push(c),
                    KeyCode::Backspace => {
                        self.input.pop();
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn handle_normal_mode(&mut self, key: &KeyEvent) -> () {
        match key.code {
            KeyCode::Char('q') => self.mode = AppMode::Exit,
            KeyCode::Char('i') => self.mode = AppMode::Input,
            _ => (),
        }
    }
}
