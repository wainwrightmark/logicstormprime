#![feature(iter_next_chunk)]
#![feature(extend_one)]
#![feature(once_cell)]
#![feature(is_sorted)]
#![feature(step_trait)]

use app::App;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod app;
pub mod state;
pub mod letter;
pub mod keyboard;
pub mod slots;
pub mod header;
pub mod answers;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
