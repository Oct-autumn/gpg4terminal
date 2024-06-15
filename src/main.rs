mod app;
mod event_handler;
mod theme;
mod ui;

use std::io::{self, stdout};

use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;

fn main() -> Result<(), io::Error> {
    // Enable raw mode and enter alternate screen
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
        )
    )?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    App::default().run(&mut terminal)?;

    // Restore terminal state
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    execute!(stdout(), PopKeyboardEnhancementFlags)?;
    terminal.show_cursor()?;

    Ok(())
}
