mod application;
mod renderer;

fn main() {
    let mut ui = renderer::Renderer::new();
    ui.start().expect("count not render UI");
}
