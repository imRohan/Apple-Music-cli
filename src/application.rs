use std::process::{Command, Stdio};

pub struct AppleMusic {
    pub status: Status,
}

pub struct NowPlaying {
    pub title: String,
    pub artist: String,
    pub album: String,
}

pub enum Status {
    NotRunning,
    PLAYING,
    PAUSED,
    STOPPED,
}

impl AppleMusic {
    fn run_command(command: &str) -> String {
        let script = format!(r#"tell application "Music" to {}"#, command);
        AppleMusic::execute_osascript(&script)
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

    pub fn new() -> AppleMusic {
        AppleMusic {
            status: Status::NotRunning,
        }
    }

    pub fn start(&mut self) {
        let script = r#"open application "Music""#;
        AppleMusic::execute_osascript(script);
        self.status = Status::STOPPED;
    }

    pub fn play(&mut self) {
        AppleMusic::run_command("play");
        self.status = Status::PLAYING;
    }

    pub fn pause(&mut self) {
        AppleMusic::run_command("pause");
        self.status = Status::PAUSED;
    }

    pub fn now_playing(&mut self) -> NowPlaying {
        let title = AppleMusic::run_command("get name of current track");
        let artist = AppleMusic::run_command("get artist of current track");
        let album = AppleMusic::run_command("get album of current track");
        NowPlaying { title: title, artist: artist, album: album }
    }
}
