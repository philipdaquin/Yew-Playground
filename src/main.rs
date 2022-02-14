mod app;
mod api;
mod types;
mod service;
pub mod components;
mod hooks;
pub mod router;
pub mod contexts;
pub mod higherordercomp;
pub mod renderprop;
pub mod use_effect;




use crate::app::App;

use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}