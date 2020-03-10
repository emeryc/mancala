#![recursion_limit = "256"]

extern crate yew;
mod components;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::console;

use components::App;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world! - 1"));
    yew::start_app::<App>();
    console::log_1(&JsValue::from_str("Hello world! - 2"));
    Ok(())
}
