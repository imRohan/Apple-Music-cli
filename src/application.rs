use std::process::{Command, Stdio};

pub struct AppleMusic {
}

pub struct NowPlaying {
    pub title: String,
    pub artist: String,
    pub album: String,
}

impl AppleMusic {
    fn run_command(command: &str) -> String {
        let script = format!(r#"tell application "Music" to {}"#, command);
        let output = AppleMusic::execute_osascript(&script);
        output.trim().to_string()
    }

    fn execute_osascript(command: &str) -> String {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(command)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute osa script");
        String::from_utf8(output.stdout).unwrap()
    }

    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&mut self) {
        let script = r#"open application "Music""#;
        AppleMusic::execute_osascript(script);
    }

    pub fn play(&mut self) {
        AppleMusic::run_command("play");
    }

    pub fn pause(&mut self) {
        AppleMusic::run_command("pause");
    }

    pub fn track_position(&mut self) -> u16 {
        let position = self.current_player_position();
        let duration = self.current_track_duration();
        let percentage = ((position / duration) * 100.00) as u16;
        percentage

    }

    pub fn current_player_position(&mut self) -> f32 {
        let position: f32 = AppleMusic::run_command("player position").
            trim().
            parse().
            unwrap();
        position
    }

    pub fn current_track_duration(&mut self) -> f32 {
        let position: f32 = AppleMusic::run_command("duration of current track").
            trim().
            parse().
            unwrap();
        position
    }

    pub fn now_playing(&mut self) -> NowPlaying {
        let title = AppleMusic::run_command("get name of current track");
        let artist = AppleMusic::run_command("get artist of current track");
        let album = AppleMusic::run_command("get album of current track");
        NowPlaying { title: title, artist: artist, album: album }
    }

    pub fn state_icon(&mut self) -> String {
        if self.is_playing() {
            "▶️".to_string()
        } else {
            "⏸️".to_string()
        }
    }

    pub fn state(&mut self) -> String {
        AppleMusic::run_command("player state")
    }

    pub fn is_playing(&mut self) -> bool {
        let playing = "playing";
        self.state() == playing
    }

    pub fn stopped(&mut self) -> bool {
        let stopped_state = "stopped";
        self.state() == stopped_state
    }
}
