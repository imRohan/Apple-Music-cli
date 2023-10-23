mod application;
mod terminal;

fn main() {
    let mut apple_music = application::AppleMusic::new();
    apple_music.start();
    terminal::start(&mut apple_music).
        expect("count not render UI");
}
