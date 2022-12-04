mod app;
mod components;
mod pages;

use wee_alloc::WeeAlloc;

pub use self::app::App;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;
