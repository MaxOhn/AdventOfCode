#[macro_use]
extern crate tracing;

mod app;
mod components;
mod day;
mod year;

pub use app::App;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/years.rs"));
}
