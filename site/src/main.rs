use aoc_site::App;
use log::Level;
use wasm_logger::Config;
use yew::Renderer;

fn main() {
    wasm_logger::init(Config::new(Level::Trace));
    Renderer::<App>::new().render();
}
