use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};

use crate::application::AppleMusic;

fn handle_events(character: char) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char(character) {
                return Ok(true);
            }
       }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, apple_music: &mut AppleMusic) {
    frame.render_widget(
        Paragraph::new(apple_music.now_playing().title),
        frame.size(),
    );
}

pub fn start(apple_music: &mut AppleMusic) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, apple_music))?;
        should_quit = handle_events('q')?;
        apple_music.play();
    }

    apple_music.pause();
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
