mod app;
mod board;

pub(crate) use self::app::{App, Msg};

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::console;

pub(crate) fn log(msg: &str) {
    console::log_1(&JsValue::from_str(msg));
}
