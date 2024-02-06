use std::default;

use anyhow::Result;
use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::{Block, Borders, Paragraph},
};

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

// App state
struct App {
    counter: i64,
    should_quit: bool,
}

// App ui render function
fn ui(app: &App, f: &mut Frame) {
    let [left, right] = Layout::new(
        Direction::Horizontal,
        [Constraint::Fill(1), Constraint::Fill(1)],
    )
    .areas(f.size());
    let block = Block::default().title("User").borders(Borders::all());
    f.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter)).block(block.clone()),
        left,
    );
    f.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter))
            .wrap(Default::default())
            .block(block),
        right,
    );
}

// App update function
fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

async fn run() -> Result<()> {
    // ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // application state
    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    loop {
        // application update
        update(&mut app)?;

        // application render
        t.draw(|f| {
            ui(&app, f);
        })?;

        // application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // setup terminal
    startup()?;

    let result = run().await;

    // teardown terminal before unwrapping Result of app run
    shutdown()?;

    result?;

    Ok(())
}
