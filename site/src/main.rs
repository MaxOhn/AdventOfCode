use log::Level;
use wasm_logger::Config;
use yew::Renderer;

fn main() {
    wasm_logger::init(Config::new(Level::Trace));
    Renderer::<aoc_site::App>::new().render();
}
