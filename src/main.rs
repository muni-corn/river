#![recursion_limit = "512"]

mod app;
mod components;
mod task;

use wasm_bindgen::prelude::*;
use std::panic;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
fn main() -> Result<(), JsValue> {
    // set console panic hook
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
