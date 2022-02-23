mod app;
pub mod components;
pub mod router;
pub mod custom_hooks;

use crate::app::App;

use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}