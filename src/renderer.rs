use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};

use crate::application::AppleMusic;

pub struct Renderer {
    pub application: AppleMusic,
    pub should_quit: bool,
    pub pause_music: bool,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            application: AppleMusic::new(),
            should_quit: false,
            pause_music: false,
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        self.setup()?;
        while !self.should_quit { 
            terminal.draw(|f| self.ui(f))?;
            self.handle_inputs()?;
            self.update_application_state();
        }
        self.teardown()
    }

    fn setup(&mut self) -> io::Result<()> {
        self.application.start();
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        self.application.play();
        Ok(())
    }

    fn teardown(&mut self) -> io::Result<()> {
        self.application.pause();
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_inputs(&mut self) -> io::Result<()> {
        self.should_quit = self.handle_events('q')?;
        if self.handle_events('p')? {
            self.pause_music = !self.pause_music
        }
        Ok(())
    }

    fn update_application_state(&mut self) {
        if self.pause_music {
            self.application.pause()
        } else {
            self.application.play()
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(frame.size());
        frame.render_widget(
            Block::new().borders(Borders::TOP).title(self.application.now_playing().title),
            main_layout[0],
        );
        frame.render_widget(
            Block::new().borders(Borders::TOP).title(self.application.status_icon()),
            main_layout[2],
        );

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);
        frame.render_widget(
            Block::default().borders(Borders::ALL).title("Left"),
            inner_layout[0],
        );
        frame.render_widget(
            Block::default().borders(Borders::ALL).title("Right"),
            inner_layout[1],
        );
    }

    fn handle_events(&mut self, character: char) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char(character) {
                    return Ok(true);
                }
           }
        }
        Ok(false)
    }
}
