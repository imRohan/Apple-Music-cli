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
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            application: AppleMusic::new(),
            should_quit: false,
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        self.setup()?;
        while !self.should_quit { 
            terminal.draw(|f| self.ui(f))?;
            self.handle_inputs()?;
        }
        self.teardown()
    }

    fn setup(&mut self) -> io::Result<()> {
        self.application.start();
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
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
        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
            ])
            .split(frame.size()
        );
        let label = Span::styled(
            self.title(),
            Style::new()
                .white()
                .add_modifier(Modifier::BOLD),
        );
        let progress_bar = Gauge::default()
            .label(label)
            .gauge_style(self.guage_background())
            .percent(self.song_percent());
        frame.render_widget(progress_bar, main_layout[0]);
    }

    fn title(&mut self) -> String {
        let now_playing = self.application.now_playing();
        let player_state = self.application.state_icon();

        if self.application.stopped() {
            "No Track Playing".to_string()
        } else {
            format!("{}  {}", player_state, now_playing.title)
        }
    }

    fn song_percent(&mut self) -> u16 {
        if self.application.stopped() {
            100
        } else {
            self.application.track_position()
        }
    }

    fn guage_background(&mut self) -> Style {
        if self.application.stopped() {
            Style::new().red()
        } else if self.application.is_playing() {
            Style::new().light_green()
        } else {
            Style::new().light_blue()
        }
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
