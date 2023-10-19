use std::process::Command;

fn main() {
    let command = r#"tell application "Music" to play"#;
    Command::new("osascript")
        .arg("-e")
        .arg(command)
        .output()
        .expect("failed to execute process");
}
