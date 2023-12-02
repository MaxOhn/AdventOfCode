#[macro_use]
extern crate tracing;

mod app;
mod components;
mod day;
mod year;

pub use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/years.rs"));
}
