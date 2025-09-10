use anyhow::Result;
use crossterm::{cursor, event, execute, terminal};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{Stdout, stdout};
use tokio::time::Duration;

mod app;
mod db;
mod model;
// mod sync;
// mod timeutil;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    // init DB & app state
    let mut app = app::App::init().await?;

    // setup terminal
    let mut stdout = stdout();
    setup_terminal(&mut stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // main loop
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // non-blocking input poll
        if event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if app.on_key(key).await? {
                    break;
                }
            }
        }

        app.tick()?;
    }

    restore_terminal(std::io::stdout())?;
    Ok(())
}

fn setup_terminal(stdout: &mut Stdout) -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;
    Ok(())
}

fn restore_terminal(mut stdout: Stdout) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(&mut stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}
