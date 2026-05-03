mod app;
mod config;
mod llm;
mod ui;
use llm::{LlmService, Provider};

use anyhow::Result;
use app::{App, AppMode};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let llm_service = LlmService::new(Provider::Ollama);
    let mut app = App::new(llm_service);

    while app.mode != AppMode::Exit {
        terminal.draw(|f| ui::draw(f, &app))?;
        app.update().await?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
