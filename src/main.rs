mod application;
mod renderer;

use renderer::Renderer;

fn main() {
    let mut ui = Renderer::new();
    ui.start().expect("count not render UI");
}
